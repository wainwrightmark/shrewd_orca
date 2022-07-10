use itertools::{Itertools, MultiProduct};
use smallvec::SmallVec;
use std::{
    collections::{BTreeMap, HashMap},
    future::Future,
    iter::{FlatMap, Once},
    ops::Bound,
    str::FromStr,
};

use crate::{core::prelude::*};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum WordQuery {
    Literal(String),
    PartOfSpeech(PartOfSpeech),
    Tag(WordTag),
    //ManyAny,
    Any,
    Range { min: usize, max: usize },
    Length(usize),
    Pattern(Pattern), //TODO disjunction, conjunction, part of speech, tag
}

impl WordQuery {
    pub fn is_literal(&self) -> bool {
        matches!(self, WordQuery::Literal(_))
    }
}

impl WordQuery {
    pub fn allow(&self, term: &Homograph) -> bool {
        match self {
            WordQuery::Literal(l) => term.text.eq_ignore_ascii_case(l),
            WordQuery::Any => true,
            WordQuery::Range { min, max } => term.text.len() >= *min && term.text.len() <= *max,
            WordQuery::Length(len) => term.text.len() == *len,
            WordQuery::Pattern(p) => p.allow(term),
            WordQuery::PartOfSpeech(pos) => term.meanings.iter().any(|m|m.part_of_speech == *pos),
            WordQuery::Tag(tag) => term.meanings.iter().any(|m|m.tags.contains(*tag)),
        }
    }

    pub fn solve<'a> (&'a self, dict: &'a TermDict) -> impl Iterator<Item = &'a Homograph>  +'a + Clone
    {
        //TODO use indexes in some cases

        let homographs = dict.homographs
        .iter()
        .filter(|t| self.allow(t));

        homographs
    }
}


pub struct WordQueryIter{

}

