use std::{collections::BTreeMap, default, str::FromStr};

use crate::core::prelude::*;
use crate::language::prelude::*;
use itertools::Itertools;
use num::traits::ops::inv;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use regex::Regex;
use serde::__private::de;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

pub struct Example {
    pub text: String,
    pub description: String,
}

impl Example {
    pub fn make(text: &str, description: &str) -> Self {
        Self {
            text: text.to_string(),
            description: description.to_string(),
        }
    }

    pub fn list() -> Vec<Example> {
        vec![
            Example::make("*", "Any word"),
            Example::make("#n", "Any noun"),
            Example::make("#n + 4..5", "Short nouns"),
            Example::make("an #j + @v* #n", "Short Phrase"),
            Example::make("Emma #l =a", "Find me a husband!"),
            Example::make("#f Anderson =a", "Name my baby!"),
            Example::make("#j hero =a #f #l", "Name my character!"),
            Example::make("n?u?h?y", "Cheat at crosswords"),
            Example::make("5 + c???t + *e*", "Cheat at wordle"),
        ]
    }
}
