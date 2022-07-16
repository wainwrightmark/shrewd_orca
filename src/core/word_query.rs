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
pub struct WordQuery {
    pub terms: SmallVec<[WordQueryDisjunction; 1]>,
}

impl From<WordQueryTerm> for WordQuery {
    fn from(term: WordQueryTerm) -> Self {
        let disj = term.into();
        WordQuery {
            terms: smallvec::smallvec!(disj),
        }
    }
}

impl From<WordQuery> for WordQueryTerm {
    fn from(query: WordQuery) -> Self {
        if let Ok(term) = query
            .terms
            .iter()
            .flat_map(|x| x.terms.iter())
            .exactly_one()
        {
            return term.clone();
        } else {
            return WordQueryTerm::Nested(Box::new(query));
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WordQueryDisjunction {
    pub terms: SmallVec<[WordQueryTerm; 1]>,
}

impl From<WordQueryTerm> for WordQueryDisjunction {
    fn from(term: WordQueryTerm) -> Self {
        WordQueryDisjunction {
            terms: smallvec::smallvec!(term),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum WordQueryTerm {
    Literal(Homograph),
    PartOfSpeech(PartOfSpeech),
    Tag(WordTag),
    //ManyAny,
    Any,
    Range { min: usize, max: usize },
    Length(usize),
    Pattern(Pattern),
    Nested(Box<WordQuery>),
}

impl WordQueryDisjunction {
    pub fn allow(&self, term: &Homograph) -> bool {
        self.terms.iter().any(|t| t.allow(term))
    }

    pub fn as_literal(&self) -> Option<&Homograph> {
        if let Ok(term) = self.terms.iter().exactly_one() {
            return term.as_literal();
        }
        None
    }

    pub fn count_options(&self, dict: &WordContext) -> usize {
        if let Ok(term) = self.terms.iter().exactly_one() {
            match term {
                WordQueryTerm::Literal(_) => return 1,
                WordQueryTerm::Any => return dict.term_dict.homographs.len(),
                _ => {}
            }
        }
        dict.term_dict
            .homographs
            .iter()
            .map(|x| self.allow(x))
            .count()
    }
}

impl WordQuery {
    #[auto_enum(Iterator, Clone)]
    pub fn solve<'a>(
        &'a self,
        dict: &'a TermDict,
    ) -> impl Iterator<Item = &'a Homograph> + 'a + Clone {
        //TODO use indexes in some cases
        if let Some(l) = self.as_literal() {
            return std::iter::once(l);
        }

        return dict.homographs.iter().filter(|t| self.allow(t));
    }

    pub fn allow(&self, term: &Homograph) -> bool {
        self.terms.iter().all(|t| t.allow(term))
    }

    pub fn as_literal(&self) -> Option<&Homograph> {
        if let Ok(term) = self.terms.iter().exactly_one() {
            return term.as_literal();
        }
        None
    }

    pub fn is_any(&self) -> bool {
        self.terms.iter().all(|x| {
            x.terms.iter().any(|x| match x {
                WordQueryTerm::Any => true,
                _ => false,
            })
        })
    }

    pub fn count_options(&self, dict: &WordContext) -> usize {
        if let Ok(term) = self.terms.iter().exactly_one() {
            return term.count_options(dict);
        }
        dict.term_dict
            .homographs
            .iter()
            .map(|x| self.allow(x))
            .count()
    }
}

impl WordQueryTerm {
    pub fn as_literal(&self) -> Option<&Homograph> {
        match self {
            WordQueryTerm::Literal(h) => Some(h),
            _ => None,
        }
    }

    pub fn allow(&self, term: &Homograph) -> bool {
        match self {
            WordQueryTerm::Literal(l) => term.text.eq_ignore_ascii_case(&l.text),
            WordQueryTerm::Any => true,
            WordQueryTerm::Range { min, max } => term.text.len() >= *min && term.text.len() <= *max,
            WordQueryTerm::Length(len) => term.text.len() == *len,
            WordQueryTerm::Pattern(p) => p.allow(term),
            WordQueryTerm::PartOfSpeech(pos) => {
                term.meanings.iter().any(|m| m.part_of_speech == *pos)
            }
            WordQueryTerm::Tag(tag) => term.meanings.iter().any(|m| m.tags.contains(*tag)),
            WordQueryTerm::Nested(nested) => nested.allow(term),
        }
    }
}
