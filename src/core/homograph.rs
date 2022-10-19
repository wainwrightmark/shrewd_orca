use std::str::FromStr;

use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Homograph {
    pub text: String,
    pub is_single_word: bool,
    pub meanings: SmallVec<[Meaning; 1]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Meaning {
    pub part_of_speech: PartOfSpeech,
    pub tags: BitFlags<WordTag>,
    pub definition: Option<&'static str>,
}

impl PartialOrd for Homograph {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.text
            .to_ascii_lowercase()
            .partial_cmp(&other.text.to_ascii_lowercase())
    }
}

impl Ord for Homograph {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.text
            .to_ascii_lowercase()
            .cmp(&other.text.to_ascii_lowercase())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Preposition,
    Interjection,
    Conjunction,
    Pronoun,
    FirstName,
    LastName,
}

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WordTag {
    Masculine,
    Feminine,
    Positive,
    Negative,
}

impl FromStr for WordTag {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        match s.to_ascii_lowercase().as_str() {
            "masculine" => Ok(WordTag::Masculine),
            "feminine" => Ok(WordTag::Feminine),
            "positive" => Ok(WordTag::Positive),
            "negative" => Ok(WordTag::Negative),

            _ => anyhow::bail!("Could not parse {} as tag", s),
        }
    }
}

impl FromStr for PartOfSpeech {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        match s.to_ascii_lowercase().as_str() {
            "n" => Ok(PartOfSpeech::Noun),
            "v" => Ok(PartOfSpeech::Verb),
            "j" => Ok(PartOfSpeech::Adjective),
            "a" => Ok(PartOfSpeech::Adverb),
            "p" => Ok(PartOfSpeech::Preposition),

            "l" => Ok(PartOfSpeech::LastName),  //TODO remove
            "f" => Ok(PartOfSpeech::FirstName), //TODO remove

            "noun" => Ok(PartOfSpeech::Noun),
            "verb" => Ok(PartOfSpeech::Verb),
            "adjective" => Ok(PartOfSpeech::Adjective),
            "adverb" => Ok(PartOfSpeech::Adverb),
            "preposition" => Ok(PartOfSpeech::Preposition),
            "firstname" => Ok(PartOfSpeech::FirstName),
            "lastname" => Ok(PartOfSpeech::LastName),

            _ => anyhow::bail!("Could not parse {} as part of speech", s),
        }
    }
}
