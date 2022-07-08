use crate::core::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use num::ToPrimitive;
use serde::*;
use std::collections::BTreeMap;
use std::default;
use std::rc::Rc;
use yewdux::prelude::*;

#[derive(PartialEq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct SavedCreationsState {
    pub creations: BTreeMap<String, Creation>,
}

impl Default for SavedCreationsState {
    fn default() -> Self {
        let mut creations = BTreeMap::new();

        for e in EXAMPLES {
            let creation = Creation {
                name: e.0.to_string(),
                text: e.1.to_string(),
            };
            creations.insert(creation.name.clone(), creation);
        }

        Self { creations }
    }
}

#[derive(PartialEq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct InputState {
    pub name: String,
    pub text: String,
    pub grammar: Grammar,
    pub overrides: BTreeMap<String, f32>,
    pub settings: ExpandSettings,
    pub error: Option<String>,
    pub seed: u64,
}

impl Default for InputState {
    fn default() -> Self {
        let example = EXAMPLES[0];
        let grammar = parse(example.1).unwrap();

        Self {
            name: example.0.to_string(),
            text: example.1.to_string(),
            grammar,
            overrides: Default::default(),
            settings: Default::default(),
            error: Default::default(),
            seed: 100,
        }
    }
}

impl InputState {
    pub fn save(&self) {
        Dispatch::<SavedCreationsState>::new().reduce_mut(|s| {
            s.creations.insert(
                self.name.clone(),
                Creation {
                    name: self.name.clone(),
                    text: self.text.clone(),
                },
            )
        });
    }

    pub fn reroll_seed(&mut self) {
        let new_seed = rand::random();
        self.seed = new_seed;
        Dispatch::<ImageState>::new().reduce_mut(|state: &mut ImageState| state.update_svg(self));
    }

    pub fn get_variable_value(&self, key: &str) -> f32 {
        if let (Some(v)) = self.overrides.get(&key.to_ascii_lowercase()) {
            *v
        } else if let Some(v) = self.grammar.defs.get(&key.to_ascii_lowercase()) {
            *v
        } else {
            0.0
        }
    }

    pub fn set_variable_value(&mut self, key: String, value: f32) {
        self.overrides.insert(key.to_ascii_lowercase(), value);
        Dispatch::<ImageState>::new().reduce_mut(|state: &mut ImageState| state.update_svg(self));
    }

    pub fn update_settings(&mut self, settings: ExpandSettings) {
        self.settings = settings;
        Dispatch::<ImageState>::new().reduce_mut(|state: &mut ImageState| state.update_svg(self));
    }

    pub fn use_creation(&mut self, name: String) {
        let saved = Dispatch::<SavedCreationsState>::new().get();
        let s = saved.creations.get(&name);

        if let Some(creation) = s {
            self.name = creation.name.clone();
            self.update_text(creation.text.clone());
        }
    }

    pub fn update_text(&mut self, new_text: String) {
        if self.text != new_text {
            self.text = new_text.clone();
            let grammar_result = parse(new_text.as_str());

            match grammar_result {
                Ok(grammar) => {
                    self.error = None;
                    if self.grammar != grammar {
                        self.grammar = grammar;
                        Dispatch::<ImageState>::new()
                            .reduce_mut(|state: &mut ImageState| state.update_svg(self));
                    }
                }
                Err(error) => self.error = Some(error),
            }
        }
    }
}
