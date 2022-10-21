use std::{collections::BTreeMap, str::FromStr};

use enumflags2::BitFlags;
use itertools::Itertools;
use serde::Deserialize;
use smallvec::SmallVec;

use crate::core::prelude::*;

#[derive(Debug)]
pub struct TermDict {
    pub homographs: Vec<Homograph>,

    pub homographs_by_part_of_speech: BTreeMap<PartOfSpeech, Vec<Homograph>>,
}

include_flate::flate!(static WORDDATATEXT: str from "src/core/WordData.tsv");

impl TermDict {
    pub fn try_find(&self, s: &str) -> Option<Homograph> {
        self.homographs.iter().find(|x| x.text == s).cloned()
    }

    pub fn from_term_data() -> Result<Self, anyhow::Error> {
        Self::from_csv(&WORDDATATEXT)
    }

    pub fn from_csv(s: &'static str) -> Result<Self, anyhow::Error> {
        let mut terms: Vec<(&str, Meaning)> = Vec::new();

        for line in s.split_terminator('\n') {
            let mut parts = line.split('\t');
            let pos_lit = parts.next().ok_or(anyhow::format_err!("Missing POS"))?;
            let text = parts.next().ok_or(anyhow::format_err!("Missing Term"))?;
            let definition_str = parts
                .next()
                .ok_or(anyhow::format_err!("Missing Definition"))?;
            let definition = if definition_str.is_empty() {
                None
            } else {
                Some(definition_str)
            };

            let mut tags: BitFlags<WordTag> = Default::default();

            if let Some(tags_str) = parts.next() {
                for tag_str in tags_str.split_ascii_whitespace() {
                    let word_tag = WordTag::from_str(tag_str)?;
                    tags.insert(word_tag);
                }
            }

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
            .iter()
            .cloned()
            .enumerate()
            .sorted_by_key(|x| x.1 .0)
            .group_by(|a| a.1 .0)
            .into_iter()
            .map(|(text, group)| {
                let mut i: Option<usize> = None;
                let meanings = SmallVec::from_iter(
                    group
                        .inspect(|x| {
                            if i.is_none() {
                                i = Some(x.0)
                            }
                        })
                        .map(|p| p.1 .1),
                );
                let homograph = Homograph {
                    text: text.to_string(),
                    is_single_word: true,
                    meanings,
                };

                (i.unwrap(), homograph)
            })
            .sorted_by_key(|(i, _)| *i)
            .map(|(_, x)| x)
            .collect_vec();

        let homographs_by_part_of_speech = terms
            .into_iter()
            .group_by(|x| x.1.part_of_speech)
            .into_iter()
            .map(|(pos, group)| {
                (
                    pos,
                    group
                        .into_iter()
                        .enumerate()
                        .sorted_by_key(|x| x.1 .0)
                        .group_by(|a| a.1 .0)
                        .into_iter()
                        .map(|(text, group)| {
                            let mut i: Option<usize> = None;
                            let meanings = SmallVec::from_iter(
                                group
                                    .inspect(|x| {
                                        if i.is_none() {
                                            i = Some(x.0)
                                        }
                                    })
                                    .map(|p| p.1 .1),
                            );
                            let homograph = Homograph {
                                text: text.to_string(),
                                is_single_word: true,
                                meanings,
                            };

                            (i.unwrap(), homograph)
                        })
                        .sorted_by_key(|(i, _)| *i)
                        .map(|(_, x)| x)
                        .collect_vec(),
                )
            })
            .collect();

        Ok(TermDict {
            homographs,
            homographs_by_part_of_speech,
        })
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
