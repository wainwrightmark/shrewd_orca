use auto_enums::auto_enum;
use itertools::{Itertools, MultiProduct};
use smallvec::SmallVec;
use std::{
    collections::{BTreeMap, HashMap},
    future::Future,
    iter::{self, FlatMap, Once},
    ops::Bound,
    str::FromStr,
};

use crate::core::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum WordQuery {
    Literal(Homograph),
    PartOfSpeech(PartOfSpeech),
    Tag(WordTag),
    //ManyAny,
    Any,
    Range { min: usize, max: usize },
    Length(usize),
    Pattern(Pattern), //TODO disjunction, conjunction, part of speech, tag
}

impl WordQuery {
    pub fn as_literal(&self)-> Option<&Homograph>{
        match self {
            WordQuery::Literal(h) => Some(h),
            _=>None
        }   
    }
}

impl WordQuery {
    pub fn allow(&self, term: &Homograph) -> bool {
        match self {
            WordQuery::Literal(l) => term.text.eq_ignore_ascii_case(&l.text),
            WordQuery::Any => true,
            WordQuery::Range { min, max } => term.text.len() >= *min && term.text.len() <= *max,
            WordQuery::Length(len) => term.text.len() == *len,
            WordQuery::Pattern(p) => p.allow(term),
            WordQuery::PartOfSpeech(pos) => term.meanings.iter().any(|m| m.part_of_speech == *pos),
            WordQuery::Tag(tag) => term.meanings.iter().any(|m| m.tags.contains(*tag)),
        }
    }

    pub fn count_options(&self, dict: &WordContext ) -> usize{
        match self {
            WordQuery::Literal(_) => 1,
            WordQuery::Any => dict.term_dict.homographs.len(),
            _=> dict.term_dict.homographs.iter().map(|x|self.allow(x)).count()
        }
    }

    #[auto_enum(Iterator, Clone)]

    pub fn solve<'a>(&'a self, dict: &'a TermDict) -> impl Iterator<Item = &'a Homograph> + 'a + Clone
    {
        //TODO use indexes in some case
        match self {
            WordQuery::Literal(l) => {
                //let h = Homograph { text: l.clone(), is_single_word: true, meanings: Default::default() };
                std::iter::once( l)
                //std::iter::empty()
            }
            _ => {
                let homographs = dict.homographs.iter().filter(|t| self.allow(t));

                homographs
            }
        }
    }
}

pub struct WordQueryIter {}
