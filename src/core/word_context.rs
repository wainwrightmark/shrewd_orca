use log::debug;

use crate::core::prelude::*;
pub struct WordContext {
    pub term_dict: TermDict,
    pub anagram_dict: AnagramDict,
}

impl WordContext {
    pub fn try_get(&self, word: &str) -> Option<&Homograph> {
        self.term_dict.homographs.iter().find(|x| x.text == word)
    }

    pub fn from_data() -> WordContext {
        debug!("Loading Word context");
        let now = instant::Instant::now();
        let term_dict = TermDict::from_term_data().unwrap();
        let anagram_dict = AnagramDict::from(term_dict.homographs.clone().into_iter());

        debug!("Loaded word context in {:?}", now.elapsed());
        WordContext {
            term_dict,
            anagram_dict,
            //phrase_expressions,
        }
    }
}
