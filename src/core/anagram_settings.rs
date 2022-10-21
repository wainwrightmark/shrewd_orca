use crate::core::prelude::*;

#[derive(Clone, Debug)]
pub struct AnagramSettings {
    pub min_word_length: u8,
    pub max_words: Option<usize>,
    //pub filter: Option<WordQuery>
}

impl Default for AnagramSettings {
    fn default() -> Self {
        Self {
            min_word_length: 3,
            max_words: 3.into(),
            //filter: None
        }
    }
}

impl AnagramSettings {
    // pub fn allow_word(&self, terms: &[Homograph]) -> bool {

    //     match &self.filter {
    //         Some(x) => terms.iter().any(|t| x.allow(t)),
    //         None => true,
    //     }
    // }

    pub fn allow_key(&self, key: &AnagramKey) -> bool {
        key.len >= self.min_word_length
    }
}
