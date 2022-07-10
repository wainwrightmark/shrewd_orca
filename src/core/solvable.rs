use std::path::Iter;

use itertools::Itertools;
use smallvec::{smallvec, SmallVec};

use crate::core::prelude::*;
use crate::language::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolveSettings {
    pub max_solutions: usize,
}


impl Default for SolveSettings{
    fn default() -> Self {
        Self { max_solutions: 10 }
    }
}


// pub trait Solvable {
//     fn solve(&self, dict: &WordContext, settings: &SolveSettings) -> Box<dyn Iterator<Item = Solution>>;
// }