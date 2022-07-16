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
            let _ = parts.next().ok_or("Missing Deinition")?;
            let definition_str = parts.next().ok_or("Missing Deinition")?;
            let definition = if definition_str.is_empty() {
                None
            } else {
                Some(definition_str.to_string())
            };

            let tags: BitFlags<WordTag> = Default::default();

            let part_of_speech = PartOfSpeech::from_str(pos_lit)?;
            let term = (
                text,
                Meaning {
                    part_of_speech,
                    tags,
                    definition,
                },
            );
            terms.push(term);
        }

        let homographs = terms
            .into_iter()
            .enumerate()
            .sorted_by_key(|x| x.1 .0.to_ascii_lowercase())
            .group_by(|a| a.1 .0.to_ascii_lowercase())
            .into_iter()
            .map(|(text, group)| {
                let mut i: Option<usize> = None;
                let meanings = SmallVec::from_iter(
                    group
                        .inspect(|x| {
                            if i == None {
                                i = Some(x.0)
                            }
                        })
                        .map(|p| p.1 .1),
                );
                let homograph = Homograph {
                    text,
                    is_single_word: true,
                    meanings,
                };

                (i.unwrap(), homograph)
            })
            .sorted_by_key(|(i, _)| *i)
            .map(|(_, x)| x)
            .collect_vec();

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
