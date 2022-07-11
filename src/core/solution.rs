use itertools::Itertools;
use smallvec::SmallVec;
use crate::language::prelude::*;
use crate::core::prelude::*;
use std::{collections::BTreeMap, default, str::FromStr};

use crate::language::prelude::*;
use num::traits::ops::inv;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuestionSolution{
    Expression(ExpressionSolution),
    Anagram(AnagramSolution)
}

impl QuestionSolution{
    pub fn get_text(&self)-> String{
        match self {
            QuestionSolution::Expression(e) => e.get_text(),
            QuestionSolution::Anagram(a) => a.get_text(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExpressionSolution{
    pub homographs: SmallVec<[Homograph; 4]>
}

impl ExpressionSolution{
    pub fn get_text(&self)-> String{
        self.homographs.iter().map(|x|x.text.as_str()).join(" ")
    }
}

impl AnagramSolution{
    pub fn get_text(&self)-> String{
        (self.left .homographs.iter().map(|x|x.text.as_str()).join(" ")
         + " : " +
         self.right .homographs.iter().map(|x|x.text.as_str()).join(" ").as_str()) 
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnagramSolution{
    pub left: ExpressionSolution,
    pub right: ExpressionSolution
}

impl AnagramSolution{
    pub fn flip(self)-> Self{
        AnagramSolution { left:self.right, right: self.left }
    }
}