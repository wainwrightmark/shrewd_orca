use itertools::{Itertools, MultiProduct};
use smallvec::SmallVec;
use std::{
    collections::{BTreeMap, HashMap},
    future::Future,
    iter::{FlatMap, Once},
    ops::Bound,
    str::FromStr,
};

use crate::{core::prelude::*};

#[derive(Clone, Copy, Debug)]
pub struct AnagramSettings {
    pub min_word_length: u8,
    pub max_words: usize,
}

impl Default for AnagramSettings {
    fn default() -> Self {
        Self {
            min_word_length: 3,
            max_words: 3,
        }
    }
}

impl AnagramSettings {
    pub fn allow(&self, key: &AnagramKey) -> bool {
        key.len >= self.min_word_length
    }
}