mod anagram_dict;
mod anagram_iterator;
mod anagram_key;
mod anagram_settings;
mod equation;
mod expression;
mod homograph;
mod pattern;
mod question;
mod solution;
mod solvable;
mod term_dict;
mod word_context;
mod word_query;

pub mod prelude {
    pub use crate::core::anagram_dict::*;
    pub use crate::core::anagram_iterator::*;
    pub use crate::core::anagram_key::*;
    pub use crate::core::anagram_settings::*;
    pub use crate::core::equation::*;
    pub use crate::core::expression::*;
    pub use crate::core::homograph::*;
    pub use crate::core::pattern::*;
    pub use crate::core::question::*;
    pub use crate::core::solution::*;
    pub use crate::core::solvable::*;
    pub use crate::core::term_dict::*;
    pub use crate::core::word_context::*;
    pub use crate::core::word_query::*;
}
