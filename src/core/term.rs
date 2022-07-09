use std::str::FromStr;

use enumflags2::{bitflags, make_bitflags, BitFlags};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Homograph{
    pub text: String,
    pub is_single_word: bool,
    pub meanings: SmallVec<[Meaning; 2]>
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Meaning {
    pub part_of_speech: PartOfSpeech,    
    pub tags: BitFlags<WordTag>,    
    pub definition: String,
}

impl PartialOrd for Homograph {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.text.partial_cmp(&other.text)
    }
}

impl Ord for Homograph {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.text.cmp(&other.text)
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
