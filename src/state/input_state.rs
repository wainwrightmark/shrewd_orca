use crate::core::prelude::*;
use crate::core::prelude::*;
use crate::language::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use log::debug;
use num::ToPrimitive;
use serde::*;

use std::collections::BTreeMap;
use std::default;
use std::rc::Rc;
use yewdux::prelude::*;

#[derive(PartialEq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct InputState {
    pub text: String,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            text: "4 5".to_string(),
        }
    }
}

impl InputState {
    pub fn change(self: &mut Self, s: String) {
        if self.text == s {
            return;
        } else {
            self.text = s;

            let r = word_lang_parse(&self.text);
            match r {
                Ok(question) => {
                    let sol = question
                        .solve(get_solve_context(), &Default::default())
                        .clone();
                    debug!("Question solved with {} solutions", sol.len());

                    Dispatch::<ResultsState>::new().set(ResultsState {
                        data: sol.into(),
                        warning: Default::default(),
                    })
                }
                Err(warning) => {
                    debug!("Warning {}", warning);

                    Dispatch::<ResultsState>::new().set(ResultsState {
                        data: Default::default(),
                        warning: Some(warning),
                    })
                }
            }
        }
    }
}
