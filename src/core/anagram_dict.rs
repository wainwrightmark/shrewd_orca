use itertools::{Itertools, MultiProduct};
use smallvec::SmallVec;
use std::{
    collections::{BTreeMap, HashMap},
    future::Future,
    iter::{FlatMap, Once},
    ops::Bound,
    str::FromStr,
};

use crate::core::prelude::*;

pub struct AnagramDict {
    pub words: BTreeMap<AnagramKey, Vec<Homograph>>,
}

impl From<TermDict> for AnagramDict {
    fn from(term_dict: TermDict) -> Self {
        let terms = term_dict.homographs;

        Self::from(terms.into_iter())
    }
}

impl<'a, T: Iterator<Item = Homograph>> From<T> for AnagramDict {
    fn from(iter: T) -> Self {
        let groups = iter
            .sorted()
            .dedup()
            .filter_map(|term| AnagramKey::from_str(&term.text).ok().map(|key| (key, term)))
            .into_group_map();
        let words = BTreeMap::from_iter(groups);

        AnagramDict { words }
    }
}

impl AnagramDict {
    pub fn solve_for_word(
        &self,
        word: &str,
        settings: SolveSettings,
    ) -> impl '_ + Iterator<Item = Vec<Homograph>> {
        let key = AnagramKey::from_str(word).unwrap();
        self.solve(key, settings)
    }

    fn solve(
        &self,
        key: AnagramKey,
        settings: SolveSettings,
    ) -> impl '_ + Iterator<Item = Vec<Homograph>> {
        let iterator = AnagramIterator::create(self, key, settings);

        

        iterator.flat_map(|solution| {
            solution
                .into_iter()
                .map(|k| self.words.get(&k).unwrap().clone()) //Note if terms with the same text, they will each be returned
                .multi_cartesian_product()
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SolveSettings {
    min_word_length: u8,
    max_words: usize,
}

impl Default for SolveSettings {
    fn default() -> Self {
        Self {
            min_word_length: 3,
            max_words: 3,
        }
    }
}

impl SolveSettings {
    pub fn allow(&self, key: &AnagramKey) -> bool {
        key.len >= self.min_word_length
    }
}

pub struct AnagramIterator<'b>
//TODO const N
{
    dict: &'b AnagramDict,
    stack: SmallVec<[(AnagramKey, Bound<AnagramKey>); 5]>,
    used_words: SmallVec<[AnagramKey; 5]>,
    settings: SolveSettings,
}

impl<'b> AnagramIterator<'b> {
    pub fn create(dict: &'b AnagramDict, key: AnagramKey, settings: SolveSettings) -> Self {
        let mut stack = SmallVec::<[(AnagramKey, Bound<AnagramKey>); 5]>::new();
        stack.push((key, Bound::Included(key)));

        Self {
            dict,
            stack,
            settings,
            used_words: Default::default(),
        }
    }
}

impl<'b> Iterator for AnagramIterator<'b> {
    type Item = SmallVec<[AnagramKey; 5]>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            let top = self.stack.last_mut().unwrap();

            if let Some((remainder, next_key)) = self
                .dict
                .words
                .range((Bound::Unbounded, top.1))
                .rev()
                .filter(|(&next_key, _)| self.settings.allow(&next_key))
                .filter_map(|(&next_key, _)| {
                    (top.0 - next_key).map(|remainder| (remainder, next_key))
                })
                .next()
            {       
                top.1 = Bound:: Excluded(next_key);
                
                if remainder.is_empty() {
                    let mut new_used = self.used_words.clone();
                    new_used.push(next_key);                    
                    return Some(new_used);
                } else if self.settings.allow(&remainder) {
                    if self.settings.max_words == self.used_words.len() + 2 {

                        if remainder <= next_key && self.dict.words.contains_key(&remainder)
                            && self.settings.allow(&remainder)
                        {
                            let mut new_used = self.used_words.clone();
                            new_used.push(next_key);
                            new_used.push(remainder);
                            return Some(new_used);
                        }
                        self.used_words.pop();
                        self.stack.pop();
                    } else {
                        self.used_words.push(next_key);
                        self.stack.push((remainder, Bound::Included(next_key) ))
                    }
                }
            } else {
                self.stack.pop();
                self.used_words.pop();
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use itertools::Itertools;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::AnagramDict;
    use super::AnagramKey;
    use crate::core::prelude::*;
    use ntest::test_case;

    #[test]
    fn test_solve_with_term_dict() {
        let term_dict = TermDict::from_term_data().unwrap();

        let dict = AnagramDict::from(term_dict);

        let solutions = dict.solve_for_word(
            "clint eastwood",
            SolveSettings {
                min_word_length: 3,
                max_words: 3,
            },
        );

        let solutions_string = solutions
            .into_iter()
            //.sorted_by_key(|x| x.len())
            .map(|s| s.into_iter().map(|t| t.text).join(" "))
            .dedup()
            .take(10)
            .join("; ");

        assert_eq!(solutions_string, "downscale Tito; wainscoted lot; colonised Watt; colonised twat; desolation cwt; lacewood stint; downcast Eliot; anecdotist Low; anecdotist owl; dislocate town")
    }

    #[test_case("i react", "act ire cat", 3, 3, 10, "act ire; cat ire", name = "basic")]
    #[test_case(
        "clint eastwood",
        "downscale Tito",
        3,
        2,
        10,
        "downscale Tito",
        name = "clint"
    )]
    #[test_case("chacha", "cha", 3, 3, 10, "cha cha", name = "repeat_word")]
    #[test_case(
        "i react",
        "act ire cat",
        3,
        2,
        10,
        "act ire; cat ire",
        name = "two_words"
    )]
    #[test_case(
        "i react",
        "act ire cat i react",
        3,
        2,
        10,
        "act ire; cat ire",
        name = "min_word_length"
    )]
    fn test_solve(
        input: String,
        terms: String,
        min_word_length: u8,
        max_words: usize,
        take: usize,
        expect: String,
    ) {
        let words = terms.split_ascii_whitespace().map(|text| Homograph {
            text: text.to_string(),
            is_single_word: true,
            meanings: Default::default()
        });

        let dict = AnagramDict::from(words);

        let solutions = dict.solve_for_word(
            input,
            SolveSettings {
                min_word_length,
                max_words,
            },
        );

        let solutions_string = solutions
            .into_iter()
            .take(take)
            .sorted_by_key(|x| x.len())
            .map(|s| s.into_iter().map(|t| t.text).join(" "))
            .join("; ");

        assert_eq!(solutions_string, expect);
    }

    #[test]
    fn test_create_dict() {
        let words = "act ire cat act ire cat"
            .split_ascii_whitespace()
            .enumerate()
            .map(|(position, text)| Homograph {
                text: text.to_string(),
                is_single_word: true,
                meanings: Default::default()
            });

        let dict = AnagramDict::from(words);

        assert_eq!(dict.words.len(), 2); //act and cat should be the same word
        let terms = dict
            .words
            .values()
            .flatten()
            .map(|x| x.text.clone())
            .join(";");
        assert_eq!(terms, "ire;ire;act;act;cat;cat")
    }
}
