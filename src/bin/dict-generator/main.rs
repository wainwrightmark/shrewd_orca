use std::collections::HashMap;
use std::fs::File;
use std::io::{Write};
use serde::Deserialize;
use quick_xml::de::{from_reader};
pub fn main(){
    let reader = quick_xml::Reader::from_file("src\\bin\\dict-generator\\english-wordnet-2021.xml").unwrap();
    let resource: LexicalResource = from_reader(reader.into_inner()).unwrap();    
    
    let synset_dic: HashMap<_,_> = resource.lexicon.synsets.iter().map(|s| (s.id.clone(), s) ).collect();


    let path = "src/core/WordData.tsv";

    let mut output = File::create(path).expect("Could not open file for writing");

    let words = resource.lexicon.lexical_entries.into_iter()
    .filter(|x|x.lemma.is_dictionary_word())
    .map(|e| (e.lemma.part_of_speech, e.lemma.written_form, 
     e.senses.iter().filter_map(|s| synset_dic[&s.synset].definition.clone()).next().unwrap_or("".to_string())   
    ));

    for (pos, text, definition,) in words{

        let part_of_speech = match pos {
            PartOfSpeech::Noun => "n",
            PartOfSpeech::Verb => "v",
            PartOfSpeech::Adjective => "j",
            PartOfSpeech::Adverb => "a",
            PartOfSpeech::AdjectiveSatellite => "j",
            PartOfSpeech::FirstName => "f",
            PartOfSpeech::LastName => "l",
        };

        write!(output, "{}\t{}\t{}\n", part_of_speech, text, definition).expect("Could not write line");
    }

    let first_names = include_str!("first-names.txt").split_ascii_whitespace().take(2500);

    for name in first_names{
        write!(output, "{}\t{}\t{}\n", "f", name, "").expect("Could not write line");
    }


    let last_names = include_str!("last-names.txt").split_ascii_whitespace().take(2500);

    for name in last_names{
        write!(output, "{}\t{}\t{}\n", "l", name, "").expect("Could not write line");
    }
    
}




#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct LexicalResource {
    #[serde(rename = "Lexicon", default)]
    pub lexicon: Lexicon

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
    #[serde( default)]
    pub url: Option<String> ,
    #[serde( default)]
    pub citation: Option<String> ,
    //TODO more fields

    #[serde(rename = "LexicalEntry", default)]
    pub lexical_entries: Vec<LexicalEntry>,
    #[serde(rename = "Synset", default)]
    pub synsets: Vec<Synset>,
    #[serde(rename = "SyntacticBehaviour", default)]
    pub behaviours: Vec<SyntacticBehaviour>


}
#[derive(Clone, Debug,  PartialEq, Deserialize)]
pub struct LexicalEntry {
    pub id: String,
    #[serde(rename = "Lemma")]
    pub lemma: Lemma,
    #[serde(rename = "Sense", default)]
    pub senses: Vec<Sense>

}

#[derive(Clone, Debug,  PartialEq, Deserialize)]
pub struct Lemma{
    #[serde(rename = "writtenForm")]
    pub written_form: String,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: PartOfSpeech,
    #[serde(rename = "Pronunciation", default)]
    pub pronunciations: Vec<Pronunciation>
}

impl Lemma{
    pub fn is_dictionary_word(&self)->bool{
        if self.written_form.len() <= 2{return false;}

        if self.written_form.chars().all(|c| c.is_ascii_alphabetic() && c.is_ascii_lowercase()){
            return true;
        }
        return false;
    }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Pronunciation{
    #[serde(rename = "variety", default)]
    pub variety: Option<String>,
    #[serde(rename = "$value")]
    pub text: String
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Sense{
    pub id: String,
    pub synset: String,
    #[serde(rename = "SenseRelation", default)]
    pub sense_relations: Vec<Relation>
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Relation{
    #[serde(rename = "relType")]
    pub rel_type: String,
    pub target: String,

}

#[derive(Clone, Debug,  PartialEq, Deserialize)]
pub struct Synset{
    pub id : String,
    pub ili: String,
    pub members: String,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: PartOfSpeech,
    #[serde(rename = "dc:subject", default)]
    pub subject: Option<String>,

    #[serde(rename = "$unflatten=Definition", default)]
    pub definition: Option<String>,

    #[serde(rename = "SynsetRelation", default)]
    pub synset_relations: Vec<Relation>

}


#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct SyntacticBehaviour{}


#[derive(Clone, Debug,  PartialEq, Deserialize)]
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