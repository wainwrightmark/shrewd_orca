use std::str::FromStr;

use enumflags2::{bitflags, make_bitflags, BitFlags};


#[derive(Debug,  Clone, PartialEq, Eq)]
pub struct Term<'a>{
    pub text: &'a str,
    pub tags: BitFlags<WordTag>,
    pub is_single_word: bool
}

impl<'a> PartialOrd for Term<'a>{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.text.partial_cmp(other.text) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.is_single_word.partial_cmp(&other.is_single_word)
    }
}

impl<'a> Ord for Term<'a>{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.text.cmp(&other.text) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.is_single_word.cmp(&other.is_single_word)
    }
}

pub enum PartOfSpeech{
    Noun,
    Verb,
    Adjective,
    Adverb,
    Article,
    Preposition
}

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WordTag{
    Masculine,
    Feminine,
    FirstName,
    LastName
}

impl FromStr for PartOfSpeech{
    type Err= String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "n" => Ok(PartOfSpeech::Noun),
            "v" => Ok(PartOfSpeech::Verb),
            "j" => Ok(PartOfSpeech::Adjective),
            "a" => Ok(PartOfSpeech::Adverb),
            "t" => Ok(PartOfSpeech::Article),
            "p" => Ok(PartOfSpeech::Preposition),


            "l" => Ok(PartOfSpeech::Noun), //TODO remove
            "f" => Ok(PartOfSpeech::Noun),//TODO remove

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