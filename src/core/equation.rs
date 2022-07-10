use itertools::{Itertools, MultiProduct};
use auto_enums::auto_enum;
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
    #[auto_enum(Iterator)]
    pub fn solve<'a> (&'a self, dict: &'a WordContext) -> impl Iterator<Item = AnagramSolution> +'a {

        if self.right.words.len() == 0{
            return std::iter::empty();
        }

        match self.operator {
            EqualityOperator::Anagram => {

                

                let lefts = self.left.solve(dict);

                lefts.flat_map(|left| dict.anagram_dict.solve_for_word(&left.get_text(), self.left.anagram_settings())
                
                .filter_map(|s| self.right.order_to_allow(s))

                .map(move |right| AnagramSolution{left: left.clone(), right})
                )
            },
        }
         //TODO
    }
}