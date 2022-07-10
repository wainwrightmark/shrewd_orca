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
pub struct WordContext {
    pub term_dict: TermDict,
    pub anagram_dict: AnagramDict,
}

impl WordContext {
    pub fn from_data() -> WordContext {
        let term_dict = TermDict::from_term_data().unwrap();
        let anagram_dict = AnagramDict::from(term_dict.homographs.clone().into_iter());

        WordContext {
            term_dict,
            anagram_dict,
        }
    }
}