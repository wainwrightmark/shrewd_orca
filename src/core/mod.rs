mod anagram_dict;
mod anagram_key;
mod anagram_iterator;
mod anagram_settings;
mod term;
mod term_dict;
mod solution;
mod word_context;
mod expression;
mod word_query;
mod solvable;
mod pattern;
mod question;
mod equation;

pub mod prelude {
    pub use crate::core::anagram_dict::*;
    pub use crate::core::anagram_key::*;
    pub use crate::core::anagram_iterator::*;
    pub use crate::core::anagram_settings::*;
    pub use crate::core::term::*;
    pub use crate::core::term_dict::*;
    pub use crate::core::solution::*;
    pub use crate::core::word_context::*;
    pub use crate::core::expression::*;
    pub use crate::core::word_query::*;
    pub use crate::core::solvable::*;
    pub use crate::core::pattern::*;
    pub use crate::core::question::*;
    pub use crate::core::equation::*;
}
