use std::rc::Rc;

use crate::core::prelude::*;
use crate::language::prelude::*;
use itertools::Itertools;
use log::debug;
use once_cell::sync::OnceCell;
use serde::*;

use yewdux::{prelude::*, storage};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct FullState {
    pub text: String,
    pub max_solutions: usize,
    #[serde(skip)]
    pub data: Rc<Vec<QuestionSolution>>,
    pub warning: Option<String>,
}

static SOLVECONTEXT: OnceCell<WordContext> = OnceCell::new();

pub fn get_solve_context() -> &'static WordContext {
    SOLVECONTEXT.get_or_init(WordContext::from_data)
}

impl Default for FullState {
    fn default() -> Self {
        Self {
            text: "4 5".to_string(),
            max_solutions: 10,
            data: Default::default(),
            warning: Default::default(),
        }
    }
}

impl Store for FullState {
    fn new() -> Self {
        init_listener(storage::StorageListener::<Self>::new(storage::Area::Local));

        let mut result: FullState = storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default();

        result.load_more();
        result
    }

    fn changed(&self, other: &Self) -> bool {
        self.text.trim() != other.text.trim() || self.max_solutions != other.max_solutions
    }
}

impl FullState {
    pub fn load_more(&mut self) {
        self.max_solutions += 10;
        self.update();
    }

    fn update(&mut self) {
        let r = question_parse(&self.text);
        match r {
            Ok(question) => {
                let start_instant = instant::Instant::now();
                let sol = question
                    .solve(get_solve_context())
                    .take(self.max_solutions)
                    .collect_vec();

                let diff = instant::Instant::now() - start_instant;
                debug!("Question solved with {} solutions in {:?}", sol.len(), diff);

                self.data = sol.into();
                self.warning = Default::default();
            }
            Err(warning) => {
                debug!("Warning {}", warning);

                self.data = Default::default();
                self.warning = Some(warning.to_string());
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
