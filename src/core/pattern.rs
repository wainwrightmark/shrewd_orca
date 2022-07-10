use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Pattern {
    pub components: Vec<PatternComponent>,
    pub regex: Regex,
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.components == other.components
    }
}

impl Eq for Pattern {}


impl Pattern {
    pub fn allow(&self, term: &Homograph) -> bool {
        self.regex.is_match(&term.text)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PatternComponent {
    Any,
    AnyChar(usize),
    Literal(String),
}

impl PatternComponent {
    pub fn regex_str(&self) -> String {
        match self {
            PatternComponent::Any => "[[:alpha:]]*".to_string(),
            PatternComponent::AnyChar(len) => format!("[[:alpha:]]{{{}}}", len),
            PatternComponent::Literal(s) => s.clone(),
        }
    }
}


