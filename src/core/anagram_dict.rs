use auto_enums::auto_enum;
use itertools::Itertools;
use smallvec::SmallVec;
use std::{collections::BTreeMap, str::FromStr};

use crate::core::prelude::*;

pub struct AnagramDict {
    pub words: BTreeMap<AnagramKey, SmallVec<[Homograph; 1]>>,
}

impl From<&TermDict> for AnagramDict {
    fn from(term_dict: &TermDict) -> Self {
        Self::from(term_dict.homographs.iter().cloned())
    }
}

impl<T: Iterator<Item = Homograph>> From<T> for AnagramDict {
    fn from(iter: T) -> Self {
        let groups = iter
            .sorted()
            .dedup()
            .filter_map(|term| AnagramKey::from_str(&term.text).ok().map(|key| (key, term)))
            .into_group_map();
        let words =
            BTreeMap::from_iter(groups.into_iter().map(|(k, g)| (k, SmallVec::from_vec(g))));

        AnagramDict { words }
    }
}

impl AnagramDict {
    #[auto_enum(Iterator)]
    pub fn solve_for_word(
        &self,
        word: &str,
        settings: AnagramSettings,
    ) -> impl '_ + Iterator<Item = ExpressionSolution> {
        if let Ok(key) = AnagramKey::from_str(word) {
            self.solve(key, settings)
        } else {
            std::iter::empty()
        }
    }

    pub fn solve(
        &self,
        key: AnagramKey,
        settings: AnagramSettings,
    ) -> impl '_ + Iterator<Item = ExpressionSolution> {
        let iterator = AnagramIterator::<4>::create(self, key, settings);

        iterator.flat_map(|solution| {
            solution
                .into_iter()
                .map(|k| self.words.get(&k).unwrap().clone()) //Note if terms with the same text, they will each be returned
                .multi_cartesian_product()
                .map(|x| ExpressionSolution {
                    homographs: SmallVec::from_vec(x),
                })
        })
    }
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::AnagramDict;
    use crate::core::prelude::*;
    use ntest::test_case;

    #[test]
    fn test_solve_with_term_dict() {
        let term_dict = TermDict::from_term_data().unwrap();

        let dict = AnagramDict::from(&term_dict);

        let solutions = dict.solve_for_word(
            "clint eastwood",
            AnagramSettings {
                min_word_length: 3,
                max_words: Some(3),
                //filter: None
            },
        );

        let solutions_string = solutions
            .into_iter()
            //.sorted_by_key(|x| x.len())
            .map(|s| s.get_text())
            .take(10)
            .join("; ");

        assert_eq!(solutions_string, "wainscoted lot; colonised twat; colonised watt; colonised Watt; desolation cwt; lacewood stint; anecdotist low; anecdotist owl; dislocate town; dislocate wont")
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
            meanings: Default::default(),
        });

        let dict = AnagramDict::from(words);

        let solutions = dict.solve_for_word(
            input,
            AnagramSettings {
                min_word_length,
                max_words: Some(max_words),
                //filter:None
            },
        );

        let solutions_string = solutions
            .into_iter()
            .take(take)
            .sorted_by_key(|x| x.homographs.len())
            .map(|s| s.get_text())
            .join("; ");

        assert_eq!(solutions_string, expect);
    }
}
