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
        if !left_expression.allow_number_of_words(2) || !right_expression.allow_number_of_words(2) {
            return std::iter::empty::<SpoonerismSolution>();
        }

        let result = left_expression
            .solve(dict)
            .filter(|x| x.homographs.len() == 2)
            .filter_map(|left| {
                let w1 = left.homographs[0].clone();
                let w2 = left.homographs[1].clone();
                let nw1 = w2.text[0..1].to_string() + &w1.text[1..];

                let h1 = dict.term_dict.try_find(nw1.as_str());
                h1.as_ref()?;

                let nw2 = w1.text[0..1].to_string() + &w2.text[1..];

                let h2 = dict.term_dict.try_find(nw2.as_str());
                h2.as_ref()?;

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

        return result;
    }

    #[auto_enum(Iterator)]
    fn solve_anagram<'a>(
        left: &'a Expression,
        right: &'a Expression,
        dict: &'a WordContext,
    ) -> impl Iterator<Item = AnagramSolution> + 'a {
        let lefts = left.solve(dict);

        if let Expression::FixedLength(right_fixed_length) = right {
            let right_literals = right_fixed_length
                .words
                .iter()
                .enumerate()
                .filter_map(|(i, query)| query.as_literal().map(|l| (l, i)))
                .collect_vec();

            if !right_literals.is_empty() {
                if let Ok(key_to_subtract) = AnagramKey::from_str(
                    right_literals
                        .clone()
                        .into_iter()
                        .map(|(x, _)| x.text.clone())
                        .join("")
                        .as_str(),
                ) {
                    let new_right_words = right_fixed_length
                        .words
                        .iter()
                        .filter(|x| x.as_literal().is_none())
                        .cloned()
                        .collect_vec();

                    let new_right = Expression::FixedLength(FixedLengthExpression {
                        words: new_right_words,
                    });

                    let settings = new_right.to_anagram_settings();

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
        let left_options = self.left.count_options(dict).unwrap_or(usize::MAX);
        if left_options == 0 {
            return std::iter::empty();
        }

        let right_options = self.right.count_options(dict).unwrap_or(usize::MAX);
        if right_options == 0 {
            return std::iter::empty();
        }

        let left_first = {
            match left_options.cmp(&right_options) {
                std::cmp::Ordering::Less => true,
                std::cmp::Ordering::Equal => match (&self.left, &self.right) {
                    (Expression::Many(l), Expression::Many(r)) => {
                        l.terms.len() >= r.terms.len() //choose the one with more term restrictions
                    } //the one with fewer options goes first
                    (Expression::Many(_), Expression::FixedLength(_)) => true, //fle should be right
                    (Expression::FixedLength(_), Expression::Many(_)) => false, //fle should be right
                    (Expression::FixedLength(l), Expression::FixedLength(r)) => {
                        match l.count_literal_chars().cmp(&r.count_literal_chars()) {
                            //the one with more literal chars should be on the right
                            std::cmp::Ordering::Less => true,
                            std::cmp::Ordering::Greater => false,
                            std::cmp::Ordering::Equal => true, //left goes first by default
                        }
                    }
                },
                std::cmp::Ordering::Greater => false,
            }
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
            EqualityOperator::Anagram => self.solve_as_anagram(dict).map(QuestionSolution::Anagram),
            EqualityOperator::Spoonerism => self
                .solve_as_spoonerism(dict)
                .map(QuestionSolution::Spoonerism),
        }
    }

    // pub fn solve_anagram_phrase<'a>(
    //     expression: &'a Expression,
    //     dict: &'a WordContext,
    // ) -> impl Iterator<Item = AnagramSolution> + 'a {
    //     expression.solve(dict).flat_map(move |solution| {
    //         dict.phrase_expressions.iter().flat_map(move |right| {
    //             Equation {
    //                 left: Expression::FixedLength(solution.clone().into()),
    //                 right: right.clone(),
    //                 operator: EqualityOperator::Anagram,
    //             }
    //             .solve_as_anagram(dict)
    //             .collect_vec()
    //         })
    //     })
    // }
}
