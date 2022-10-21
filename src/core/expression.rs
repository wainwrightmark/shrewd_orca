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
        if let Expression::Many(m) = self {
            return m.solve(dict);
        }

        if let Expression::FixedLength(fl) = self {
            return fl.solve(dict);
        }

        unreachable!()
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
