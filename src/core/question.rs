use crate::core::prelude::*;
use itertools::Itertools;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Question {
    Expression(Expression),
    Equation(Equation),
}

impl Question {
    pub fn solve(&self, dict: &WordContext, settings: &SolveSettings) -> Vec<QuestionSolution> {
        match self {
            Question::Expression(ex) => {
                // if ex.words.iter().all(|w| w.as_literal().is_some()) {
                //     let text = ex
                //         .words
                //         .iter()
                //         .map(|wq| wq.as_literal().unwrap().text.clone())
                //         .join("");

                //     if text.is_empty() {
                //         return Default::default();
                //     }
                //     dict.anagram_dict
                //         .solve_for_word(text.as_str(), Default::default())
                //         .take(settings.max_solutions)
                //         .map(QuestionSolution::Expression)
                //         .collect_vec()
                // } else {
                    
                // }
                ex.solve(dict)
                        .take(settings.max_solutions)
                        .map(QuestionSolution::Expression)
                        .collect_vec()
            }

            Question::Equation(eq) => eq
                .solve(dict)
                .take(settings.max_solutions)                
                .collect_vec(),
        }
    }
}
