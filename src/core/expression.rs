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
pub struct Expression {
    pub words: Vec<WordQuery>,
}



impl Expression{
    pub fn allow(solution: &Vec<Homograph>)-> bool{
        todo!()
    }
}


impl Expression {
    pub fn solve<'a> (&'a self, dict: &'a WordContext) -> impl Iterator<Item = Solution> +'a {
        let solutions = self
                .words
                .iter()
                .map(|w| w.solve(&dict.term_dict))
                .multi_cartesian_product()
                .map(|homographs| Solution{homographs: SmallVec::from_iter(homographs.into_iter().cloned()) })
                ;

            solutions
    }
}