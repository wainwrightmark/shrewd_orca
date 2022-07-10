use std::str::FromStr;

use enumflags2::BitFlags;
use itertools::Itertools;
use serde::Deserialize;
use smallvec::SmallVec;

use crate::core::prelude::*;

#[derive(Debug)]
pub struct TermDict {
    pub homographs: Vec<Homograph>,
}

impl TermDict {
    pub fn from_term_data() -> Result<Self, String> {
        let txt = include_str!("WordData.tsv");
        Self::from_csv(txt)
    }

    pub fn from_csv(s: &str) -> Result<Self, String> {
        let mut terms: Vec<(&str, Meaning)> = Vec::new();

        for line in s.split_terminator('\n') {
            let mut parts = line.split('\t');
            let pos_lit = parts.next().ok_or("Missing POS")?;
            let text = parts.next().ok_or("Missing Term")?;
            let a_key = parts.next().ok_or("Missing Deinition")?;
            let definition = parts.next().ok_or("Missing Deinition")?;

            let tags: BitFlags<WordTag> = Default::default();

            let part_of_speech = PartOfSpeech::from_str(pos_lit)?;
            let term =
            (text,
            Meaning{
                part_of_speech,
                tags,
                definition: definition.to_string(),
            }
            );
            terms.push(term);
        }

        terms.sort_by_key(|x|x.0.to_ascii_lowercase());
        let homographs = terms
        .into_iter()
        .group_by(|a|a.0.to_ascii_lowercase())
        .into_iter().map(|x| Homograph{
            text: x.0,
            is_single_word: true,
            meanings: SmallVec::from_iter(x.1.map(|p|p.1))

        }).collect_vec()
        ;

        Ok(TermDict { homographs })
    }
}

#[derive(Debug, Deserialize)]
pub struct CPTerm<'a> {
    //TODO change this when we've generated out own
    pub pos: &'a str,
    pub text: &'a str,
    pub key: &'a str,
    pub def: &'a str,
}
