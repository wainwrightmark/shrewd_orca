use std::str::FromStr;

use enumflags2::BitFlags;
use serde::Deserialize;

use super::term::{PartOfSpeech, Term, WordTag};

#[derive(Debug)]
pub struct TermDict {
    pub terms: Vec<Term>,
}

impl TermDict {
    pub fn from_term_data() -> Result<Self, String> {
        let txt = include_str!("WordData.tsv");
        Self::from_csv(txt)
    }

    pub fn from_csv(s: &str) -> Result<Self, String> {
        let mut terms = Vec::new();

        for line in s.split_terminator('\n') {
            let mut parts = line.split('\t');
            let pos_lit = parts.next().ok_or("Missing POS")?;
            let text = parts.next().ok_or("Missing Term")?;
            let a_key = parts.next().ok_or("Missing Deinition")?;
            let definition = parts.next().ok_or("Missing Deinition")?;

            let tags: BitFlags<WordTag> = match pos_lit {
                "f" => WordTag::FirstName.into(),
                "l" => WordTag::LastName.into(),
                _ => Default::default(),
            };

            let part_of_speech = PartOfSpeech::from_str(pos_lit)?;
            let term = Term {
                part_of_speech,
                text: text.to_string(),
                is_single_word: true,
                tags,
                definition: definition.to_string(),
            };
            terms.push(term);
        }
        Ok(TermDict { terms })
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
