use crate::core::prelude::*;
use itertools::Itertools;
use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Pattern {
    pub components: Vec<PatternComponent>,
    pub regex: Regex,
}

impl From<Vec<PatternComponent>> for Pattern {
    fn from(components: Vec<PatternComponent>) -> Self {
        let regex_str =
            "^(?i)".to_owned() + &components.iter().map(|x| x.regex_str()).join("") + "$";

        let regex = Regex::new(regex_str.as_str()).unwrap();

        Pattern { components, regex }
    }
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
    CharacterClass(CharacterClass),
}

impl PatternComponent {
    pub fn regex_str(&self) -> String {
        match self {
            PatternComponent::Any => "[[:alpha:]]*".to_string(),
            PatternComponent::AnyChar(len) => format!("[[:alpha:]]{{{len}}}"),
            PatternComponent::Literal(s) => s.clone(),
            PatternComponent::CharacterClass(c) => c.regex_char().to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CharacterClass {
    Vowel,
    Consonant,
}
impl CharacterClass {
    pub fn regex_char(&self) -> &'static str {
        match self {
            CharacterClass::Vowel => "[aeiou]",
            CharacterClass::Consonant => "[bcdfghjklmnpqrstvwxyz]",
        }
    }
}

impl FromStr for CharacterClass {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        match s.to_ascii_lowercase().as_str() {
            "@v" => Ok(CharacterClass::Vowel),
            "@c" => Ok(CharacterClass::Consonant),
            _ => anyhow::bail!("The only valid character classes are @v and @c"),
        }
    }
}
