use itertools::Itertools;
use quick_xml::de::from_reader;
use serde::Deserialize;
use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::Write;

pub fn main() {
    let reader = quick_xml::Reader::from_file("src\\bin\\dict-generator\\english-wordnet-2021.xml")
        .expect("Could not read English Wordnet file. You may need to download this");
    let resource: LexicalResource = from_reader(reader.into_inner()).unwrap();

    let synset_dic: HashMap<_, _> = resource
        .lexicon
        .synsets
        .iter()
        .map(|s| (s.id.clone(), s))
        .collect();

    let words_path = "src/core/WordData.tsv";

    let mut words_output = File::create(words_path).expect("Could not open file for writing");

    let positive_words: BTreeSet<String> = include_str!("positive-words.txt")
        .lines()
        .map(|x| x.to_ascii_lowercase())
        .collect();

    let negative_words: BTreeSet<String> = include_str!("negative-words.txt")
        .lines()
        .map(|x| x.to_ascii_lowercase())
        .collect();

    let mut words = resource
        .lexicon
        .lexical_entries
        .into_iter()
        .filter(|x| x.lemma.is_dictionary_word())
        .map(|e| {
            let mut tags_vec = vec![];
            if positive_words.contains(&e.lemma.written_form.to_ascii_lowercase()) {
                tags_vec.push("positive")
            } else if negative_words.contains(&e.lemma.written_form.to_ascii_lowercase()) {
                tags_vec.push("negative")
            }
            Word {
                part_of_speech: e.lemma.part_of_speech,
                lemma: e.lemma.written_form,
                definition: e
                    .senses
                    .iter()
                    .filter_map(|s| synset_dic[&s.synset].definition.clone())
                    .next()
                    .unwrap_or_default(),
                tags: tags_vec.join(" "),
            }
        })
        .collect_vec();

    let boys_names = include_str!("boys-names.txt").split_ascii_whitespace();

    let girls_names = include_str!("girls-names.txt").split_ascii_whitespace();

    let first_names = boys_names
        .map(|name| Word {
            part_of_speech: PartOfSpeech::FirstName,
            lemma: name.to_string(),
            definition: "".to_string(),
            tags: "masculine".to_string(),
        })
        .take(1000)
        .interleave(girls_names.map(|name| Word {
            part_of_speech: PartOfSpeech::FirstName,
            lemma: name.to_string(),
            definition: "".to_string(),
            tags: "feminine".to_string(),
        }))
        .take(1000);

    words.extend(first_names);

    let last_names = include_str!("last-names.txt")
        .split_ascii_whitespace()
        .take(2500)
        .map(|name| Word {
            part_of_speech: PartOfSpeech::LastName,
            lemma: name.to_string(),
            definition: "".to_string(),
            tags: "".to_string(),
        });

    words.extend(last_names);

    for word in words {
        writeln!(
            words_output,
            "{}\t{}\t{}\t{}",
            word.part_of_speech.to_str(),
            word.lemma,
            word.definition,
            word.tags
        )
        .expect("Could not write line");
    }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct LexicalResource {
    #[serde(rename = "Lexicon", default)]
    pub lexicon: Lexicon,
}
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]

pub struct Lexicon {
    pub id: String,
    pub label: String,
    pub language: String,
    pub email: String,
    pub license: String,
    pub version: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub citation: Option<String>,
    //TODO more fields
    #[serde(rename = "LexicalEntry", default)]
    pub lexical_entries: Vec<LexicalEntry>,
    #[serde(rename = "Synset", default)]
    pub synsets: Vec<Synset>,
    #[serde(rename = "SyntacticBehaviour", default)]
    pub behaviours: Vec<SyntacticBehaviour>,
}
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct LexicalEntry {
    pub id: String,
    #[serde(rename = "Lemma")]
    pub lemma: Lemma,
    #[serde(rename = "Sense", default)]
    pub senses: Vec<Sense>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Lemma {
    #[serde(rename = "writtenForm")]
    pub written_form: String,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: PartOfSpeech,
    #[serde(rename = "Pronunciation", default)]
    pub pronunciations: Vec<Pronunciation>,
}

impl Lemma {
    pub fn is_dictionary_word(&self) -> bool {
        if self.written_form.len() <= 2 {
            return false;
        }

        if self
            .written_form
            .chars()
            .all(|c| c.is_ascii_alphabetic() && c.is_ascii_lowercase())
        {
            return true;
        }
        false
    }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Pronunciation {
    #[serde(rename = "variety", default)]
    pub variety: Option<String>,
    #[serde(rename = "$value")]
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Sense {
    pub id: String,
    pub synset: String,
    #[serde(rename = "SenseRelation", default)]
    pub sense_relations: Vec<Relation>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Relation {
    #[serde(rename = "relType")]
    pub rel_type: String,
    pub target: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Synset {
    pub id: String,
    pub ili: String,
    pub members: String,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: PartOfSpeech,
    #[serde(rename = "dc:subject", default)]
    pub subject: Option<String>,

    #[serde(rename = "$unflatten=Definition", default)]
    pub definition: Option<String>,

    #[serde(rename = "SynsetRelation", default)]
    pub synset_relations: Vec<Relation>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct SyntacticBehaviour {}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum PartOfSpeech {
    #[serde(rename = "n")]
    Noun,
    #[serde(rename = "v")]
    Verb,
    #[serde(rename = "a")]
    Adjective,
    #[serde(rename = "r")]
    Adverb,

    #[serde(rename = "s")]
    AdjectiveSatellite,

    #[serde(rename = "f")]
    FirstName,
    #[serde(rename = "l")]
    LastName,
}

impl PartOfSpeech {
    pub fn to_str(&self) -> &'static str {
        use PartOfSpeech::*;
        match self {
            Noun => "n",
            Verb => "v",
            Adjective => "j",
            Adverb => "a",
            AdjectiveSatellite => "j",
            FirstName => "f",
            LastName => "l",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Word {
    pub part_of_speech: PartOfSpeech,
    pub lemma: String,
    pub definition: String,
    pub tags: String,
}
