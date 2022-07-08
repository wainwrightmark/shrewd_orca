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

pub struct AnagramDict<'a> {
    pub words: BTreeMap<AnagramKey, Vec<Term<'a>>>,
}

impl<'a> From<TermDict<'a>> for AnagramDict<'a> {
    fn from(term_dict: TermDict<'a>) -> Self {
        let terms = term_dict.terms;

        Self::from(terms.into_iter())
    }
}

impl<'a, T: Iterator<Item = Term<'a>>> From<T> for AnagramDict<'a> {
    fn from(iter: T) -> Self {
        let groups = iter
            .sorted()
            .dedup()
            .filter_map(|term| AnagramKey::from_str(term.text).ok().map(|key| (key, term)))
            .into_group_map();
        let words = BTreeMap::from_iter(groups);

        AnagramDict { words }
    }
}

impl<'a> AnagramDict<'a> {
    fn solve_for_word(
        &self,
        word: &str,
        settings: SolveSettings,
    ) -> impl '_ + Iterator<Item = Vec<Term>> {
        let key = AnagramKey::from_str(word).unwrap();
        self.solve(key, settings)
    }

    fn solve(
        &self,
        key: AnagramKey,
        settings: SolveSettings,
    ) -> impl '_ + Iterator<Item = Vec<Term>> {
        let iterator = AnagramIterator::create(self, key, settings);

        let solutions = iterator.flat_map(|solution| {
            solution
                .into_iter()
                .map(|k| self.words.get(&k).unwrap().clone()) //Note if terms with the same text, they will each be returned
                .multi_cartesian_product()
        });

        solutions
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

pub struct AnagramIterator<'a, 'b>
//TODO const N
{
    dict: &'b AnagramDict<'a>,
    stack: SmallVec<[(AnagramKey, AnagramKey); 5]>,
    used_words: SmallVec<[AnagramKey; 5]>,
    settings: SolveSettings,
}

impl<'a, 'b> AnagramIterator<'a, 'b> {
    pub fn create(dict: &'b AnagramDict<'a>, key: AnagramKey, settings: SolveSettings) -> Self {
        let mut stack = SmallVec::<[(AnagramKey, AnagramKey); 5]>::new();
        stack.push((key, AnagramKey::EMPTY));

        Self {
            dict,
            stack,
            settings,
            used_words: Default::default()
        }
    }
}

impl<'a, 'b> Iterator for AnagramIterator<'a, 'b> {
    type Item = SmallVec<[AnagramKey; 5]>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            let current_len = self.used_words.len();
            let (current_key, previous) = self.stack.last_mut().unwrap();

            if previous >= current_key {
                //todo check previous squared
                self.stack.pop();
                self.used_words.pop();
                continue;
            }

            if let Some((remainder, next_key)) = self
                .dict
                .words
                .range((Bound::Excluded(*previous), Bound::Included(*current_key)))
                .filter(|(&next_key, _)| self.settings.allow(&next_key))
                .filter_map(|(&next_key, _)| {
                    (*current_key - next_key).map(|remainder| (remainder, next_key))
                })
                .next()
            {
                previous.inner = next_key.inner;

                if remainder.is_empty() {
                    let mut new_used = self.used_words.clone();
                    new_used.push(next_key);
                    self.used_words.pop();
                    self.stack.pop();
                    return Some(new_used);
                } else if next_key > remainder {
                    // if the remainder is in the dictionary, we have already passed it
                } else if self.settings.allow(&remainder)
                    && self.settings.max_words > current_len
                {
                    self.used_words.push(next_key);
                    self.stack.push((
                        remainder,
                        AnagramKey {
                            inner: next_key.inner - 1,
                            len: next_key.len,
                        },
                    ))
                }
            } else {
                self.stack.pop();
                self.used_words.pop();
            }
        }

        return None;
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

    
    #[test]
    fn test_solve_with_term_dict() {
        let term_dict = TermDict::from_term_data().unwrap();

        let dict = AnagramDict::from(term_dict);

        let solutions = dict.solve_for_word("clint eastwood", Default::default());

        let solutions_string = solutions
            .into_iter()
            .sorted_by_key(|x| x.len())
            .map(|s| s.into_iter().map(|t| t.text).join(" "))
            .dedup()
            .take(10)            
            .join("; ");

        assert_eq!(solutions_string, "Tito downscale; lot wainscoted; Watt colonised; twat colonised; cwt desolation; stint lacewood; Eliot downcast; Low anecdotist; owl anecdotist; town dislocate")
    }

    #[test]
    fn test_solve_basic() {
        let words = "act ire cat".split_ascii_whitespace().map(|text| Term {
            part_of_speech: PartOfSpeech::Noun,
            text,
            tags: Default::default(),
            is_single_word: true,
        });

        let dict = AnagramDict::from(words);

        let solutions = dict.solve_for_word("i react", Default::default());

        let solutions_string = solutions
            .into_iter()
            .sorted_by_key(|x| x.len())
            .map(|s| s.into_iter().map(|t| t.text).join(" "))
            .join("; ");

        assert_eq!(solutions_string, "ire act; ire cat");
    }

    #[test]
    fn test_create_dict(){
        let words = "act ire cat act ire cat".split_ascii_whitespace().enumerate() .map(|(position, text)| Term {
            part_of_speech: if position < 3{PartOfSpeech::Noun} else{PartOfSpeech::Verb} ,
            text,
            tags: Default::default(),
            is_single_word: true,
        });

        let dict = AnagramDict::from(words);

        assert_eq!(dict.words.len(), 2); //act and cat should be the same word
        let terms = dict.words.values().flat_map(|x|x).map(|x|x.text).join(";");
        assert_eq!(terms, "ire;ire;act;act;cat;cat")

    }

    #[test]
    fn test_duplicate_word() {
        let words = "cha".split_ascii_whitespace().map(|text| Term {
            part_of_speech: PartOfSpeech::Noun,
            text,
            tags: Default::default(),
            is_single_word: true,
        });

        let dict = AnagramDict::from(words);

        let solutions = dict.solve_for_word("chacha", Default::default());

        let solutions_string = solutions
            .into_iter()
            .sorted_by_key(|x| x.len())
            .map(|s| s.into_iter().map(|t| t.text).join(" "))
            .join("; ");

        assert_eq!(solutions_string, "cha cha");
    }
}
