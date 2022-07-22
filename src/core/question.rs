use crate::core::prelude::*;
use auto_enums::auto_enum;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Question {
    Expression(Expression),
    Equation(Equation),
}

impl Question {
    #[auto_enum(Iterator)]
    pub fn solve<'a> (&'a self, dict: &'a WordContext) -> impl Iterator<Item = QuestionSolution> + 'a {
        match self {
            Question::Expression(ex) => {
                ex.solve(dict)
                        .map(QuestionSolution::Expression)
            }

            Question::Equation(eq) => eq
                .solve(dict),
        }
    }
}
