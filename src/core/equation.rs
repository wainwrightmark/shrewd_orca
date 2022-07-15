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

use super::homograph;

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
        left: &'a Expression,
        right: &'a Expression,
        dict: &'a WordContext,
    ) -> impl Iterator<Item = AnagramSolution> + 'a {
        if right.words.len() == 0 {
            return std::iter::empty();
        }

        let lefts = left.solve(dict);
        let right_literals = right
            .words
            .iter()
            .enumerate()
            .filter_map(|(i, query)| query.as_literal().map(|l| (l, i)))
            .collect_vec();

        if right_literals.is_empty() {
            let settings = right.anagram_settings();

            let s = lefts.flat_map(move |left| {
                dict.anagram_dict
                    .solve_for_word(&left.get_text(), settings)
                    .filter_map(|s| right.order_to_allow(s))
                    .map(move |right| AnagramSolution {
                        left: left.clone(),
                        right,
                    })
            }).filter(|x|!x.is_trivial());
            return s;
        } else {
            let new_right = Expression {
                words: right
                    .words
                    .iter()
                    .filter(|x| x.as_literal().is_none())
                    .cloned()
                    .collect_vec(),
            };

            if new_right.words.is_empty() {
                return std::iter::empty();
            }

            let settings = new_right.anagram_settings();

            let key_to_subtract = AnagramKey::from_str(
                right_literals
                    .clone()
                    .into_iter()
                    .map(|(x, i)| x.text.clone())
                    .join("")
                    .as_str(),
            )
            .unwrap(); //todo handle this potential error

            //let right_homographs = right_literals.iter().map(|(x,_)|x).cloned().collect_vec();

            return lefts
            //.filter(move |left| !right_homographs.iter() .any(|r| left.contains_word(r)))
                .flat_map(move |left| {
                    AnagramKey::from_str(left.get_text().as_str())
                        .ok()
                        .and_then(|k| k - key_to_subtract)
                        .map(|k| (left, k))
                })
                .flat_map(move |(left, key)| {
                    dict.anagram_dict
                        .solve(key, settings.clone())
                        .map(move |r| (left.clone(), r))
                })
                

                .filter_map(move |(left, s)| new_right.order_to_allow(s).map(|r| (left, r)))
                .map(move |(left, extra_rights)| AnagramSolution {
                    left,
                    right: Equation::hydrate(extra_rights, &right_literals),
                })
                .filter(|x|!x.is_trivial())
                ;
        };
    }

    fn hydrate(
        mut dehydrated: ExpressionSolution,
        literals: &Vec<(&Homograph, usize)>,
    ) -> ExpressionSolution {
        for (element, index) in literals {
            dehydrated
                .homographs
                .insert(index.clone(), element.clone().clone())
        }

        dehydrated
    }

    #[auto_enum(Iterator)]
    pub fn solve<'a>(
        &'a self,
        dict: &'a WordContext,
    ) -> impl Iterator<Item = AnagramSolution> + 'a {
        let left_first: bool;


        let left_options =  self.left.count_options(dict);
        let right_options = self.right.count_options(dict);
        let left_literal_count = self.left.count_literal_chars();
        let right_literal_count = self.right.count_literal_chars();

        if left_options < right_options{
            left_first = true;
        }
        else if right_options < left_options{
            left_first = false;
        }
        else{
            left_first = right_literal_count >= left_literal_count;
        }

        if left_first {
            Equation::solve_anagram(&self.left, &self.right, dict)
        } else {
            Equation::solve_anagram(&self.right, &self.left, dict).map(|x| x.flip())
        }
    }
}