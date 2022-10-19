use auto_enums::auto_enum;
use itertools::Itertools;

use std::str::FromStr;

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
    Spoonerism,
}

impl Equation {
    #[auto_enum(Iterator)]
    fn solve_spoonerism<'a>(
        left_expression: &'a Expression,
        right_expression: &'a Expression,
        dict: &'a WordContext,
    ) -> impl Iterator<Item = SpoonerismSolution> + 'a {
        if left_expression.words.len() != 2 {
            return std::iter::empty();
        } else if right_expression.words.len() != 2 {
            return std::iter::empty();
        } else {
            let result = left_expression
                .solve(dict)
                .filter(|x| x.homographs.len() == 2)
                //.map(|x| (x.homographs[0], x.homographs[1] ))
                .filter_map(|left| {
                    let w1 = left.homographs[0].clone();
                    let w2 = left.homographs[1].clone();
                    let nw1 = w2.text[0..1].to_string() + &w1.text[1..];

                    let h1 = dict.term_dict.try_find(nw1.as_str());
                    if h1.is_none() {
                        return None;
                    }

                    let nw2 = w1.text[0..1].to_string() + &w2.text[1..];

                    let h2 = dict.term_dict.try_find(nw2.as_str());
                    if h2.is_none() {
                        return None;
                    }

                    let right_homographs = smallvec::smallvec![h1.unwrap(), h2.unwrap()];

                    let right = ExpressionSolution {
                        homographs: right_homographs,
                    };

                    if !right_expression.allow(&right) {
                        return None;
                    }
                    let solution = SpoonerismSolution { left, right };

                    if solution.is_trivial() {
                        return None;
                    }

                    Some(solution)
                });

            result
        }
    }

    #[auto_enum(Iterator)]
    fn solve_anagram<'a>(
        left: &'a Expression,
        right: &'a Expression,
        dict: &'a WordContext,
    ) -> impl Iterator<Item = AnagramSolution> + 'a {
        if right.words.is_empty() {
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
            let settings = right.to_anagram_settings();

            let s = lefts
                .flat_map(move |left| {
                    dict.anagram_dict
                        .solve_for_word(&left.get_text(), settings.clone())
                        .filter_map(|s| right.order_to_allow(s))
                        .map(move |right| AnagramSolution {
                            left: left.clone(),
                            right,
                        })
                })
                .filter(|x| !x.is_trivial());
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

            let settings = new_right.to_anagram_settings();

            if let Ok(key_to_subtract) = AnagramKey::from_str(
                right_literals
                    .clone()
                    .into_iter()
                    .map(|(x, _)| x.text.clone())
                    .join("")
                    .as_str(),
            ) {
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
                            .map(move |r| (left.clone(), r))
                    })
                    .filter_map(move |(left, s)| new_right.order_to_allow(s).map(|r| (left, r)))
                    .map(move |(left, extra_rights)| AnagramSolution {
                        left,
                        right: Equation::hydrate(extra_rights, &right_literals),
                    })
                    .filter(|x| !x.is_trivial());
            }
            return std::iter::empty();
        }
    }

    fn hydrate(
        mut dehydrated: ExpressionSolution,
        literals: &Vec<(&Homograph, usize)>,
    ) -> ExpressionSolution {
        for (element, index) in literals {
            dehydrated.homographs.insert(*index, (*element).clone())
        }

        dehydrated
    }

    #[auto_enum(Iterator)]
    fn solve_as_anagram<'a>(
        &'a self,
        dict: &'a WordContext,
    ) -> impl Iterator<Item = AnagramSolution> + 'a {
        if self.right.words.is_empty() {
            return Equation::solve_anagram_phrase(&self.left, dict);
        }

        if self.left.words.is_empty() {
            return Equation::solve_anagram_phrase(&self.right, dict).map(|x| x.flip());
        }

        let left_options = self.left.count_options(dict);
        let left_literal_count = self.left.count_literal_chars();

        let right_options = self.right.count_options(dict);
        let right_literal_count = self.right.count_literal_chars();

        let left_first = match left_options.cmp(&right_options) {
            std::cmp::Ordering::Less => true,
            std::cmp::Ordering::Equal => right_literal_count >= left_literal_count,
            std::cmp::Ordering::Greater => false,
        };

        if left_first {
            Equation::solve_anagram(&self.left, &self.right, dict)
        } else {
            Equation::solve_anagram(&self.right, &self.left, dict).map(|x| x.flip())
        }
    }

    fn solve_as_spoonerism<'a>(
        &'a self,
        dict: &'a WordContext,
    ) -> impl Iterator<Item = SpoonerismSolution> + 'a {
        Equation::solve_spoonerism(&self.left, &self.right, dict)
    }

    #[auto_enum(Iterator)]
    pub fn solve<'a>(
        &'a self,
        dict: &'a WordContext,
    ) -> impl Iterator<Item = QuestionSolution> + 'a {
        match self.operator {
            EqualityOperator::Anagram => self
                .solve_as_anagram(dict)
                .map(|x| QuestionSolution::Anagram(x)),
            EqualityOperator::Spoonerism => self
                .solve_as_spoonerism(dict)
                .map(|x| QuestionSolution::Spoonerism(x)),
        }
    }

    pub fn solve_anagram_phrase<'a>(
        expression: &'a Expression,
        dict: &'a WordContext,
    ) -> impl Iterator<Item = AnagramSolution> + 'a {
        expression.solve(dict).flat_map(move |solution| {
            dict.phrase_expressions.iter().flat_map(move |right| {
                Equation {
                    left: solution.clone().into(),
                    right: right.clone(),
                    operator: EqualityOperator::Anagram,
                }
                .solve_as_anagram(dict)
                .collect_vec()
            })
        })
    }
}
