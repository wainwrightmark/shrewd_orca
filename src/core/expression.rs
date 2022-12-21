use auto_enums::auto_enum;
use enum_dispatch::enum_dispatch;

use crate::core::prelude::*;

#[enum_dispatch(TypedExpression)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    Many(ManyExpression),
    FixedLength(FixedLengthExpression),
}

impl Expression {
    #[auto_enum(Iterator)]
    pub fn solve<'a>(
        &'a self,
        dict: &'a WordContext,
    ) -> impl Iterator<Item = ExpressionSolution> + 'a {
        match self {
            Expression::Many(m) => m.solve(dict),
            Expression::FixedLength(fl) => fl.solve(dict),
        }
    }

    /// Upgrade all literals so they have definitions
    pub fn upgrade_literals(&mut self, dict: &WordContext) {
        match self {
            Expression::Many(m) => m.upgrade_literals(dict),
            Expression::FixedLength(fl) => fl.upgrade_literals(dict),
        }
    }
}

#[enum_dispatch]
pub trait TypedExpression {
    fn to_anagram_settings(&self) -> AnagramSettings;

    fn count_options(&self, dict: &WordContext) -> Option<usize>;

    fn order_to_allow(&self, solution: ExpressionSolution) -> Option<ExpressionSolution>;

    fn allow(&self, solution: &ExpressionSolution) -> bool;

    fn allow_number_of_words(&self, number_of_words: usize) -> bool;
}
