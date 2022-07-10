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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Equation {
    pub left: Expression,
    pub operator: EqualityOperator,
    pub right: Expression,
}



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EqualityOperator {
    Anagram,
}


impl  Equation {
    pub fn solve<'a> (&'a self, dict: &'a WordContext) -> impl Iterator<Item = AnagramSolution> +'a {
        match self.operator {
            EqualityOperator::Anagram => {
                let lefts = self.left.solve(dict);

                lefts.flat_map(|left| dict.anagram_dict.solve_for_word(&left.get_text(), self.left.anagram_settings())
                
                .filter(|s| self.right.accept(&s))

                .map(move |right| AnagramSolution{left: left.clone(), right})
                )
            },
        }
         //TODO
    }
}