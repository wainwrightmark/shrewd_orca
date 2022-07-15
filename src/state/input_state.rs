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
    pub max_solutions: usize
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            text: "4 5".to_string(),
            max_solutions: 10
        }
    }
}

impl InputState {

    pub fn load_more(&mut self){
        self.max_solutions += 10;
        self.update();       
    }

    fn update(&mut self){
        let r = question_parse(&self.text);
        match r {
            Ok(question) => {
                let start_instant = instant::Instant::now();
                let sol = question
                    .solve(get_solve_context(), &SolveSettings { max_solutions:self.max_solutions })
                    ;

                let diff = instant::Instant::now() - start_instant;
                debug!("Question solved with {} solutions in {:?}", sol.len(), diff);

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

    pub fn change(&mut self, s: String) {
        if self.text.trim() == s.trim() {
            
        } else {
            self.text = s;
            self.max_solutions = 10;

            self.update();
        }
    }
}
