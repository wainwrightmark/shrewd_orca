use auto_enums::auto_enum;
use itertools::Itertools;

use smallvec::SmallVec;

use std::{rc::Rc, str::FromStr};

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
    const EASY_OPTIONS: usize = 100000;
    pub fn is_too_difficult(&self, dict: &WordContext) -> bool {
        match self.operator {
            EqualityOperator::Anagram => {
                let left_options = self.left.count_options(dict).unwrap_or(usize::MAX);
                if left_options <= Self::EASY_OPTIONS {
                    return false;
                }

                let right_options = self.right.count_options(dict).unwrap_or(usize::MAX);
                if right_options <= Self::EASY_OPTIONS {
                    return false;
                }

                //info!("left: {}, right: {}", left_options, right_options);
                true
            }
            EqualityOperator::Spoonerism => false,
        }
    }

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
    fn solve_anagram_dehydrated<'a>(
        left: ExpressionSolution,
        key_to_subtract: AnagramKey,
        dehydrated_right: Rc<FixedLengthExpression>,
        extracted_literals: Rc<SmallVec<[(Homograph, usize); 2]>>,
        dict: &'a WordContext,
    ) -> impl Iterator<Item = AnagramSolution> + 'a {
        if let Some(key) = AnagramKey::from_str(left.get_text().as_str())
            .ok()
            .and_then(|k| k - key_to_subtract)
        {
            let settings = dehydrated_right.to_anagram_settings();
            let lefts = dict
                .anagram_dict
                .solve(key, settings)
                .map(move |r| (left.clone(), r))
                .filter_map(move |(left, s)| dehydrated_right.order_to_allow(s).map(|r| (left, r)))
                .map(move |(left, extra_rights)| AnagramSolution {
                    left,
                    right: Equation::hydrate(extra_rights, &extracted_literals),
                })
                .filter(|x| !x.is_trivial());

            return lefts;
        }

        return std::iter::empty::<AnagramSolution>();
    }

    #[auto_enum(Iterator)]
    fn solve_anagram<'a>(
        left: &'a Expression,
        right: &'a Expression,
        dict: &'a WordContext,
    ) -> impl Iterator<Item = AnagramSolution> + 'a {
        if let Expression::FixedLength(right_fixed_length) = right {
            if let Some((dehydrated_right, key_to_subtract, extracted_literals)) =
                right_fixed_length.extract_literals()
            {
                let rc_dr = Rc::from(dehydrated_right);
                let rc_ex_l = Rc::from(extracted_literals);

                return left.solve(dict).flat_map(move |left| {
                    Self::solve_anagram_dehydrated(
                        left,
                        key_to_subtract,
                        Rc::clone(&rc_dr),
                        Rc::clone(&rc_ex_l),
                        dict,
                    )
                });
            }
        } else if let Expression::Many(right_as_many) = right {
            if right_as_many.t == ManyExpressionType::Phrase {
                let dehydrated_rights: Rc<Vec<_>> = PHRASEEXPRESSIONS
                    .iter()
                    .filter(|pe| right_as_many.allow_number_of_words(pe.words.len()))
                    .map(|pe| {
                        if let Some((new_expression, key, vec)) = pe.extract_literals() {
                            (Rc::from(new_expression), key, Rc::from(vec))
                        } else {
                            (
                                Rc::from(pe.clone()),
                                AnagramKey::empty(),
                                Rc::from(smallvec::smallvec![]),
                            )
                        }
                    })
                    .collect_vec()
                    .into();

                return left.solve(dict).flat_map(move |left| {
                    let results = dehydrated_rights
                        .clone()
                        .iter()
                        .flat_map(
                            move |(dehydrated_right, key_to_subtract, extracted_literals)| {
                                Self::solve_anagram_dehydrated(
                                    left.clone(),
                                    *key_to_subtract,
                                    Rc::clone(dehydrated_right),
                                    Rc::clone(extracted_literals),
                                    dict,
                                )
                                .filter(|solution| {
                                    right_as_many.terms.iter().all(|t| {
                                        solution.right.homographs.iter().any(|h| t.allow(h))
                                    })
                                })
                            },
                        )
                        .collect_vec();

                    results
                });
            }
        }

        let settings = right.to_anagram_settings();

        let s = left
            .solve(dict)
            .flat_map(move |left| {
                dict.anagram_dict
                    .solve_for_word(&left.get_text(), settings)
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
        literals: &SmallVec<[(Homograph, usize); 2]>,
    ) -> ExpressionSolution {
        for (element, index) in literals {
            dehydrated.homographs.insert(*index, element.clone())
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

    pub fn upgrade_literals(&mut self, dict: &WordContext) {
        self.left.upgrade_literals(dict);
        self.right.upgrade_literals(dict);
    }
}
