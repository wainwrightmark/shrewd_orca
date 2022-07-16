#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolveSettings {
    pub max_solutions: usize,
}

impl Default for SolveSettings {
    fn default() -> Self {
        Self { max_solutions: 10 }
    }
}
