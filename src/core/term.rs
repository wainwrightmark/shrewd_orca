use std::str::FromStr;

use enumflags2::{bitflags, make_bitflags, BitFlags};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Term {
    pub part_of_speech: PartOfSpeech,
    pub text: String,
    pub tags: BitFlags<WordTag>,
    pub is_single_word: bool,
    pub definition: String,
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.text.partial_cmp(&other.text) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.is_single_word.partial_cmp(&other.is_single_word)
    }
}

impl Ord for Term {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.text.cmp(&other.text) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.is_single_word.cmp(&other.is_single_word)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Article,
    Preposition,
}

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WordTag {
    Masculine,
    Feminine,
    FirstName,
    LastName,
}

impl FromStr for PartOfSpeech {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "n" => Ok(PartOfSpeech::Noun),
            "v" => Ok(PartOfSpeech::Verb),
            "j" => Ok(PartOfSpeech::Adjective),
            "a" => Ok(PartOfSpeech::Adverb),
            "t" => Ok(PartOfSpeech::Article),
            "p" => Ok(PartOfSpeech::Preposition),

            "l" => Ok(PartOfSpeech::Noun), //TODO remove
            "f" => Ok(PartOfSpeech::Noun), //TODO remove

            "noun" => Ok(PartOfSpeech::Noun),
            "verb" => Ok(PartOfSpeech::Verb),
            "adjective" => Ok(PartOfSpeech::Adjective),
            "adverb" => Ok(PartOfSpeech::Adverb),
            "article" => Ok(PartOfSpeech::Article),
            "preposition" => Ok(PartOfSpeech::Preposition),

            _ => Err(format!("Could not parse {} as part of speech", s)),
        }
    }
}