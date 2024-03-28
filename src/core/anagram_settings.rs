use crate::core::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct AnagramSettings {
    pub min_word_length: u8,
    pub max_words: Option<usize>,
}

impl Default for AnagramSettings {
    fn default() -> Self {
        Self {
            min_word_length: 3,
            max_words: 3.into(),
        }
    }
}

impl AnagramSettings {
    pub fn allow_key(&self, key: &AnagramKey) -> bool {
        key.is_length_at_least(self.min_word_length)
    }
}
