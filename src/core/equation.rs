use auto_enums::auto_enum;
use itertools::{Itertools, MultiProduct};
use smallvec::SmallVec;
use std::{
    collections::{BTreeMap, HashMap},
    future::Future,
    iter::{FlatMap, Once},
    ops::Bound,
    str::FromStr,
};

use crate::core::prelude::*;

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

impl Equation {


    #[auto_enum(Iterator)]
    fn solve_anagram<'a>(
         left:&'a Expression,
        right:&'a  Expression,
        dict: &'a WordContext,
    ) -> impl Iterator<Item = AnagramSolution> + 'a {
        if right.words.len() == 0 {
            return std::iter::empty();
        }

        let lefts = left.solve(dict);
        let right_literals = 
            right
            .words
            .iter()
            .filter_map(|x| x.as_literal())
            .collect_vec();

        if right_literals.is_empty() {
            let settings = right.anagram_settings();

            let s = lefts.flat_map(move |left| {
                dict.anagram_dict
                    .solve_for_word(&left.get_text(),settings)
                    .filter_map(|s| right.order_to_allow(s))
                    .map(move |right| AnagramSolution {
                        left: left.clone(),
                        right,
                    })
            });
            return s;
        } else {
            let new_right = Expression {
                words: 
                    right
                    .words
                    .iter()
                    .filter(|x| x.as_literal().is_none())
                    .cloned()
                    .collect_vec(),
            };

            if new_right.words.is_empty(){
                return std::iter::empty();
            }

            let settings = new_right.anagram_settings();

            let key_to_subtract = AnagramKey::from_str(
                right_literals.clone()
                    .into_iter()
                    .map(|x| x.text.clone())
                    .join("")
                    .as_str(),
            )
            .unwrap(); //todo handle this potential error

            return lefts
                .flat_map(move |left| {
                    AnagramKey::from_str(left.get_text().as_str())
                        .ok()
                        .and_then(|k| k - key_to_subtract)
                        .map(|k| (left, k))
                })
                .flat_map(move |(left, key)| {
                    dict.anagram_dict
                        .solve(key, settings.clone())
                        .map(move |r|(left.clone(), r))
                        
                })
                .filter_map(move |(left,s)| new_right.order_to_allow(s).map(|r|(left, r)))
                .map(move |(left, extra_rights)|
                
                    AnagramSolution {
                        left,
                        right : ExpressionSolution{homographs: 
                            SmallVec::from_iter(
                                extra_rights.homographs.into_iter().chain(right_literals.clone().into_iter().cloned())) },
                    }
                )
                ;
        };
    }

    
    pub fn solve<'a>(
        &'a self,
        dict: &'a WordContext,
    ) -> impl Iterator<Item = AnagramSolution> + 'a {
       Equation::solve_anagram(&self.left, &self.right, dict)
    }
}
