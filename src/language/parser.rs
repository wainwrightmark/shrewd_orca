use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::{Pairs, Pair};
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[grammar = "language/wordlang.pest"]
pub struct ConvextParser;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Equation{
    left: Expression,
    operator: EqualityOperator,
    right: Expression,

}

#[derive(Clone,  PartialEq, Eq, Debug)]
pub struct Expression{

    pub words: Vec<WordQuery>
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EqualityOperator{
    Simple,
    Anagram
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum WordQuery{
    Literal(String),
    //ManyAny,
    Any,
    Range{min: usize, max: usize},
    Length(usize),
    //TODO disjunction, conjunction, part of speech, tag
}

pub trait CanParse where Self: Sized {
    fn try_parse(pair: Pair<Rule>) -> Result<Self, String>;
}


impl FromStr for EqualityOperator{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "==" => Ok(EqualityOperator::Simple),
            "=a" => Ok(EqualityOperator::Anagram),
            _ => Err(format!("Could not parse {} as equality operator", s)),
        }
    }
}

impl CanParse for Equation{
    fn try_parse(pair: Pair<Rule>) -> Result<Self, String> {
        let mut inner = pair.into_inner();
        let left = Expression::try_parse(inner.next().unwrap())?;
        let equality = EqualityOperator::try_parse(inner.next().unwrap())?;
        let right = Expression::try_parse(inner.next().unwrap())?;

        Ok(Equation{left, operator: equality, right})
    }
}
impl CanParse for EqualityOperator{
    fn try_parse(pair: Pair<Rule>) -> Result<Self, String> {        
        EqualityOperator::from_str(pair.as_str())
    }
}
impl CanParse for Expression{
    fn try_parse(pair: Pair<Rule>) -> Result<Self, String> {
        let words_result:Result<Vec<WordQuery>, String> =  pair.into_inner().map(WordQuery::try_parse).collect();
        let words = words_result?;

        Ok(Expression{words})
    }
}
impl CanParse for WordQuery{
    fn try_parse(pair: Pair<Rule>) -> Result<Self, String> {        
        let rule = pair.as_rule();

        match  rule {
            Rule::literal => Ok(WordQuery::Literal(pair.as_str().to_string())),
            //Rule::manyany => Ok(WordQuery::ManyAny),
            Rule::any => Ok(WordQuery::Any),
            Rule::length => Ok(WordQuery::Length(usize::from_str(pair.as_str()).unwrap())),
            Rule::range => {
                let mut inner  = pair.into_inner();
                let start = inner.next().unwrap();
                let dots = inner.next().unwrap();
                let end = inner.next().unwrap();

                let min = usize::from_str(start.as_str()).unwrap();
                let max = usize::from_str(end.as_str()).unwrap();

                Ok(WordQuery::Range { min, max})
            },
            _ => {
                unreachable!("unexpected rule {:?}", rule)
            }
        }
    }
}


pub fn parse(input: &str) -> Result<Equation, String> {
    let mut pairs = ConvextParser::parse(Rule::file, input).map_err(|e| e.to_string())?;    
    let next = pairs.next().unwrap();
    let result = Equation::try_parse(next);
    result
}
