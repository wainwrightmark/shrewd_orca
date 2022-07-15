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

    pub fn count_options(&self, dict: &WordContext ) -> usize{
        self.words.iter().map(|x|x.count_options(dict)) .fold(1, |a,b| a * b)
    }

    pub fn count_literal_chars(&self)->usize{
        self.words.iter().filter_map(|x|x.as_literal()).map(|x|x.text.len()).count()
    }


    pub fn order_to_allow(&self, solution: ExpressionSolution) -> Option<ExpressionSolution>{
        if solution.homographs.len() != self.words.len(){
            return None;
        }

        if self.allow(&solution){
            return Some(solution);
        }

        if !self.words.iter().all(|w|  solution.homographs.iter().any(|h| w.allow(h))){
            return None;
        }

        for combination in solution.homographs.into_iter().combinations(self.words.len()){
            for (w, h) in self.words.iter().zip(combination.iter()){
                if !w.allow(&h){
                    continue;
                }
            }
            return Some(ExpressionSolution{homographs: SmallVec::from_vec(combination)})
        }
        return None;
    }

    fn allow(&self, solution: &ExpressionSolution) -> bool{
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