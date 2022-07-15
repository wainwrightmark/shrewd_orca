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

use super::term_dict;
pub struct WordContext {
    pub term_dict: TermDict,
    pub anagram_dict: AnagramDict,
    pub phrase_expressions: Vec<Expression>
}

impl WordContext {


    pub fn try_get(&self, word: &str)-> Option<&Homograph>{
        self.term_dict.homographs.iter().find(|x|x.text == word)
    }

    pub fn from_data(phrase_expressions: Vec<Expression>) -> WordContext {
        let term_dict = TermDict::from_term_data().unwrap();
        let anagram_dict = AnagramDict::from(term_dict.homographs.clone().into_iter());

        WordContext {
            term_dict,
            anagram_dict,
            phrase_expressions
        }
    }
}