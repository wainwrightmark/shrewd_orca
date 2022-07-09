use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use crate::language::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::{Pairs, Pair};
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Clone, Debug)]
pub struct Pattern{
    pub components: Vec<PatternComponent>,
    pub regex: Regex
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.components == other.components
    }
}

impl Eq for Pattern{   
}

impl CanParse for Pattern{
    fn try_parse(pair: Pair<Rule>) -> Result<Self, String> {
        let components:Vec<PatternComponent> = pair.into_inner().map(PatternComponent::try_parse).try_collect()?;

        let regex_str = "^".to_owned() + &components.iter().map(|x|x.regex_str()).join("") + "$";

        let regex = Regex::new(regex_str.as_str()).unwrap();

        Ok(Pattern{components, regex})
    }
}

impl Pattern {
    pub fn allow(&self, term: &Term) -> bool {        
        self.regex.is_match(&term.text)
    }

}

#[derive(Clone,  PartialEq, Eq, Debug)]
pub enum PatternComponent{
    Any,
    AnyChar(usize),
    Literal(String)
}

impl PatternComponent {
    pub fn regex_str(&self)-> String{
        match self {
            PatternComponent::Any => "[[:alpha:]]*".to_string(),
            PatternComponent::AnyChar(len) => format!("[[:alpha:]]{{{}}}", len),
            PatternComponent::Literal(s) => s.clone(),
        }
    }
}

impl CanParse for PatternComponent {
    fn try_parse(pair: Pair<Rule>) -> Result<Self, String> {
        match pair.as_rule(){
            Rule::question_marks => Ok(PatternComponent::AnyChar(pair.as_str().len())),
            Rule::any => Ok(PatternComponent::Any),
            
            Rule::literal => Ok(PatternComponent::Literal(pair.as_str().to_string())),
            _=> unreachable!()
        }
    }
}
