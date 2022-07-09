use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use crate::language::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::{Pairs, Pair};
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[grammar = "language/wordlang.pest"]
pub struct WordLangParser;

pub fn word_lang_parse(input: &str) -> Result<Question, String> {
    let mut pairs = WordLangParser::parse(Rule::file, input).map_err(|e| e.to_string())?;        
    let next = pairs.next().unwrap();
    let question = next.into_inner().next().unwrap();
    let result = Question::try_parse(question);
    result
}



#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Question{
    Expression(Expression),
    Equation(Equation)
}



impl CanParse for Question {
    fn try_parse(pair: Pair<Rule>) -> Result<Self, String> {
        let p = pair.into_inner().next().unwrap();
        match p.as_rule(){
            Rule::equation => Ok(Question::Equation(Equation::try_parse(p)?)),
            Rule::expression => Ok(Question::Expression(Expression::try_parse(p)?)),
            rul => unreachable!("Reached {:?}", rul)
        }
    }
}

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
    Anagram
}

#[derive(Clone,  PartialEq, Eq, Debug)]
pub enum WordQuery{
    Literal(String),
    //ManyAny,
    Any,
    Range{min: usize, max: usize},
    Length(usize),
    Pattern(Pattern)
    //TODO disjunction, conjunction, part of speech, tag
}

impl WordQuery {
    pub fn is_literal(&self)-> bool{
        matches!(self, WordQuery:: Literal(_))
    }
}

pub trait CanParse where Self: Sized {
    fn try_parse(pair: Pair<Rule>) -> Result<Self, String>;
}


impl FromStr for EqualityOperator{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
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
        let inner = pair.into_inner().next().unwrap();
        let rule = inner.as_rule();
        let s = inner.as_str();

        match  rule {
            Rule::literal => Ok(WordQuery::Literal(s.to_string())),
            //Rule::manyany => Ok(WordQuery::ManyAny),
            Rule::any => Ok(WordQuery::Any),
            Rule::length =>{
                
                Ok(WordQuery::Length(usize::from_str(s).unwrap()))
            } ,
            Rule::range => {
                let mut range_inner  = inner.into_inner();

                let start = range_inner.next().unwrap();
                let end = range_inner.next().unwrap();

                let min = usize::from_str(start.as_str()).unwrap();
                let max = usize::from_str(end.as_str()).unwrap();

                Ok(WordQuery::Range { min, max})
            },
            Rule::pattern =>{
                let pattern = Pattern::try_parse(inner)?;
                Ok(WordQuery::Pattern(pattern))
            }
            _ => {
                unreachable!("unexpected rule {:?}", rule)
            }
        }
    }
}



