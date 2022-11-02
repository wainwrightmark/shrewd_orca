use auto_enums::auto_enum;
use include_flate::lazy_static;
use itertools::Itertools;
use smallvec::SmallVec;

use crate::core::prelude::*;
use crate::language::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ManyExpression {
    pub t: ManyExpressionType,
    pub terms: SmallVec<[WordQueryTerm; 1]>,
    pub min_words: usize,
    pub max_words: Option<usize>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ManyExpressionType {
    Any,
    Phrase,
}

impl ManyExpression {
    #[auto_enum(Iterator)]
    pub fn solve<'a>(
        &'a self,
        dict: &'a WordContext,
    ) -> impl Iterator<Item = ExpressionSolution> + 'a {
        match self.t {
            ManyExpressionType::Any => MANYANYEXPRESSIONS
                .iter()
                .filter(|x| self.allow_number_of_words(x.words.len()))
                .flat_map(|x| x.solve(dict))
                .filter(|x| self.allow(x)),
            ManyExpressionType::Phrase => PHRASEEXPRESSIONS
                .iter()
                .filter(|pe| self.allow_number_of_words(pe.words.len()))
                .flat_map(|x| x.solve(dict))
                .filter(|x| self.allow(x)),
        }
    }

    pub fn count_literal_chars(&self) -> usize {
        self.terms
            .iter()
            .filter_map(|x| match x {
                WordQueryTerm::Literal(l) => Some(l.text.len()),
                _ => None,
            })
            .max()
            .unwrap_or(0)
    }
}

impl TypedExpression for ManyExpression {
    fn allow_number_of_words(&self, num_words: usize) -> bool {
        if self.min_words > num_words {
            return false;
        }
        match self.max_words {
            Some(max) => num_words <= max,
            None => true,
        }
    }

    fn to_anagram_settings(&self) -> AnagramSettings {
        AnagramSettings {
            min_word_length: 3,
            max_words: match self.t {
                ManyExpressionType::Any => self.max_words,
                ManyExpressionType::Phrase => self
                    .max_words
                    .min(PHRASEEXPRESSIONS.iter().map(|x| x.words.len()).max()),
            },
        }
    }

    fn count_options(&self, dict: &WordContext) -> Option<usize> {
        match self.t {
            ManyExpressionType::Any => match self.max_words {
                Some(max) => {
                    let mut r: usize = 0;

                    for p in self.min_words..max {
                        let o = dict.term_dict.homographs.len().checked_pow(p as u32);
                        match o {
                            Some(s) => match r.checked_add(s) {
                                Some(r2) => r = r2,
                                None => return None,
                            },
                            None => return None,
                        }
                    }

                    Some(r)
                }
                None => None,
            },
            ManyExpressionType::Phrase => PHRASEEXPRESSIONS
                .iter()
                .filter(|x| self.allow_number_of_words(x.words.len()))
                .map(|x| x.count_options(dict))
                .sum(),
        }
    }

    fn order_to_allow(&self, solution: ExpressionSolution) -> Option<ExpressionSolution> {
        //log::debug!("Testing {:?} for expression {:?}", solution, self);

        if !self.allow_number_of_words(solution.homographs.len()) {
            return None;
        }
        if !self
            .terms
            .iter()
            .all(|t| solution.homographs.iter().any(|h| t.allow(h)))
        {
            return None;
        }

        match self.t {
            ManyExpressionType::Any => Some(solution),
            ManyExpressionType::Phrase => PHRASEEXPRESSIONS
                .iter()
                .filter_map(|pe| pe.order_to_allow(solution.clone())) //TODO remove clone here
                .next(),
        }
    }

    fn allow(&self, solution: &ExpressionSolution) -> bool {
        if !self.allow_number_of_words(solution.homographs.len()) {
            return false;
        }
        if !self
            .terms
            .iter()
            .all(|t| solution.homographs.iter().any(|h| t.allow(h)))
        {
            return false;
        }

        match self.t {
            ManyExpressionType::Any => true,
            ManyExpressionType::Phrase => PHRASEEXPRESSIONS.iter().any(|pe| pe.allow(solution)),
        }
    }
}

impl ManyExpressionType {
    pub fn allow(&self, solution: &ExpressionSolution) -> bool {
        //log::info!("Possible Solution: {:?}", solution);

        match self {
            ManyExpressionType::Any => true,
            ManyExpressionType::Phrase => PHRASEEXPRESSIONS.iter().any(|fle| fle.allow(solution)),
        }
    }
}

lazy_static! {
    pub static ref MANYANYEXPRESSIONS: Vec<FixedLengthExpression> = {
        let any_word_query = WordQuery {
            terms: smallvec::smallvec![WordQueryDisjunction {
                terms: smallvec::smallvec![WordQueryTerm::Any]
            }],
        };

        (1..=1)
            .map(|n| FixedLengthExpression {
                words: vec![any_word_query.clone(); n],
            })
            .collect_vec()
    };
}

lazy_static! {
    //TODO phrase expressions
    pub static ref PHRASEEXPRESSIONS: Vec<FixedLengthExpression> = {
        let expression_strings = vec![
            "*",
            "the #n",
            "#j #n",
            "a #n + @c*",
            "an #n + @v*",
            "the #j #n",
            "a #j + @c* #n",
            "an #j + @v* #n",
            "#a #v",
        ];

        expression_strings
            .into_iter()
            .map(|s| question_parse(s).unwrap())
            .map(|s| match s {
                Question::Expression(Expression::FixedLength(fle)) => fle,
                _ => unreachable!(),
            })
            .collect_vec()
    };
}
