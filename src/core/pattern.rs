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
    CharacterClass(CharacterClass)
}

impl PatternComponent {
    pub fn regex_str(&self) -> String {
        match self {
            PatternComponent::Any => "[[:alpha:]]*".to_string(),
            PatternComponent::AnyChar(len) => format!("[[:alpha:]]{{{}}}", len),
            PatternComponent::Literal(s) => s.clone(),
            PatternComponent::CharacterClass(c)=>c.regex_char()
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CharacterClass{
    Vowel,
    Consonant
}
impl CharacterClass{
    pub fn regex_char(&self)-> String{
        match self{
            CharacterClass::Vowel => "[aeiou]".to_string(),
            CharacterClass::Consonant => "[bcdfghjklmnpqrstvwxyz]".to_string(),
        }
    }
}

impl FromStr for CharacterClass{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "@v"=> Ok(CharacterClass::Vowel),
            "@c" => Ok(CharacterClass::Consonant),
            _=> Err("The only valid character classes are @v and @c".to_string())
        }
    }
}
