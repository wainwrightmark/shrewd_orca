use itertools::Itertools;
use smallvec::SmallVec;

use super::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FixedLengthExpression {
    pub words: Vec<WordQuery>,
}

impl From<ExpressionSolution> for FixedLengthExpression {
    fn from(es: ExpressionSolution) -> Self {
        let words = es
            .homographs
            .into_iter()
            .map(|h| WordQueryTerm::Literal(h).into())
            .collect_vec();
        Self { words }
    }
}

impl FixedLengthExpression {
    pub fn solve<'a>(
        &'a self,
        dict: &'a WordContext,
    ) -> impl Iterator<Item = ExpressionSolution> + 'a {
        self.words
            .iter()
            .map(|w| w.solve(&dict.term_dict))
            .multi_cartesian_product()
            .map(|homographs| ExpressionSolution {
                homographs: homographs.into_iter().cloned().collect(),
            })
    }

    pub fn count_literal_chars(&self) -> usize {
        self.words
            .iter()
            .filter_map(|x| x.as_literal())
            .map(|x| x.text.len())
            .count()
    }
}

impl TypedExpression for FixedLengthExpression {
    fn allow_number_of_words(&self, number_of_words: usize) -> bool {
        self.words.len() == number_of_words
    }

    fn to_anagram_settings(&self) -> AnagramSettings {
        AnagramSettings {
            min_word_length: 3,
            max_words: Some(self.words.len()),
        }
    }
    fn count_options(&self, dict: &WordContext) -> Option<usize> {

        if self.words.is_empty(){
            return Some(0);
        }

        Some(self.words.iter().map(|x| x.count_options(dict)).product())
    }

    fn order_to_allow(&self, solution: ExpressionSolution) -> Option<ExpressionSolution> {
        if solution.homographs.len() != self.words.len() {
            return None;
        }

        if self.allow(&solution) {
            return Some(solution);
        }

        if !self
            .words
            .iter()
            .all(|w| solution.homographs.iter().any(|h| w.allow(h)))
        {
            return None;
        }

        //log::debug!("Testing {:?} for expression {:?}", solution, self);

        'outer: for combination in solution
            .homographs
            .into_iter()
            .permutations(self.words.len())
        {
            for (w, h) in self.words.iter().zip(combination.iter()) {
                if !w.allow(h) {
                    continue 'outer;
                }
            }
            return Some(ExpressionSolution {
                homographs: SmallVec::from_vec(combination),
            });
        }
        None
    }

    fn allow(&self, solution: &ExpressionSolution) -> bool {
        if solution.homographs.len() == self.words.len() {
            for (w, h) in self.words.iter().zip(solution.homographs.iter()) {
                if !w.allow(h) {
                    return false;
                }
            }
            return true;
        }

        false
    }
}
