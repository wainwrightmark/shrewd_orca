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
            Example::make("Emma #l =a !phrase", "Find me a husband!"),
            Example::make("#f + #feminine Darcy =a !phrase + #negative", "Find me a wife!"),
            Example::make("#f + #masculine Anderson =a !phrase + #positive", "Name my baby!"),
            Example::make("#j hero =a #f #l", "Name my character!"),
            Example::make("n?u?h?y", "Cheat at crosswords"),
            Example::make("5 + c???t + *e*", "Cheat at wordle"),
        ]
    }
}
