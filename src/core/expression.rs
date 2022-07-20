use itertools::Itertools;
use smallvec::SmallVec;

use crate::core::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Expression {
    pub words: Vec<WordQuery>,
}

impl From<ExpressionSolution> for Expression {
    fn from(es: ExpressionSolution) -> Self {
        let words = es
            .homographs
            .into_iter()
            .map(|h| WordQueryTerm::Literal(h).into())
            .collect_vec();
        Expression { words }
    }
}

impl Expression {
    pub fn solve<'a>(
        &'a self,
        dict: &'a WordContext,
    ) -> impl Iterator<Item = ExpressionSolution> + 'a {
        let solutions = self
            .words
            .iter()
            .map(|w| w.solve(&dict.term_dict))
            .multi_cartesian_product()
            .map(|homographs| ExpressionSolution {
                homographs: SmallVec::from_iter(homographs.into_iter().cloned()),
            });

        solutions
    }

    pub fn to_anagram_settings(
        &self,
        // context: &WordContext
    ) -> AnagramSettings {
        AnagramSettings {
            min_word_length: 3,
            max_words: self.words.len(),
            //filter: self.as_filter(context)
        }
    }

    // pub fn as_filter(&self, context: &WordContext)-> Option<WordQuery>{

    //     if self.words.iter().any(|x|x.is_any()){
    //         return None;
    //     }
    //     if let Ok(a) = self.words.iter().exactly_one(){
    //         return Some(a.clone());
    //     }

    //     let terms = SmallVec::from_iter(self.words
    //     .iter()
    //     //.sorted_by_cached_key(|x|x.count_options(context))  .rev()
    //     .dedup()
    //     .map(|x|x.clone().into()));

    //     Some(WordQuery{terms: SmallVec::from_elem(WordQueryDisjunction{terms}, 1)})
    // }

    pub fn count_options(&self, dict: &WordContext) -> usize {
        self.words.iter().map(|x| x.count_options(dict)).product()
    }

    pub fn count_literal_chars(&self) -> usize {
        self.words
            .iter()
            .filter_map(|x| x.as_literal())
            .map(|x| x.text.len())
            .count()
    }

    pub fn order_to_allow(&self, solution: ExpressionSolution) -> Option<ExpressionSolution> {
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

    pub fn allow(&self, solution: &ExpressionSolution) -> bool {
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
