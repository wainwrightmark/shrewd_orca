use itertools::{Itertools, MultiProduct};
use smallvec::SmallVec;
use std::{
    collections::{BTreeMap, HashMap},
    future::Future,
    iter::{FlatMap, Once},
    ops::{Bound, Index},
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
    pub fn solve<'a> (&'a self, dict: &'a WordContext) -> impl Iterator<Item = ExpressionSolution> +'a {
        let solutions = self
                .words
                .iter()
                .map(|w| w.solve(&dict.term_dict))
                .multi_cartesian_product()
                .map(|homographs| ExpressionSolution{homographs: SmallVec::from_iter(homographs.into_iter().cloned()) })
                ;

            solutions
    }

    pub fn anagram_settings(&self)-> AnagramSettings{
        AnagramSettings { min_word_length: 3, max_words: self.words.len() }
    }


    pub fn accept(&self, solution: &ExpressionSolution) -> bool{
        if solution.homographs.len() == self.words.len(){

            for (w, h) in self.words.iter().zip(solution.homographs.iter()){
                if !w.allow(h){
                    return false;
                }
            }
            return true;
        }

        false
    }
}