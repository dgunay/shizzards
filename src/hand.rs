#[derive(Debug, Clone)]
pub struct Hand {
    words: Vec<String>,
}

impl Hand {
    pub fn new(words: Vec<String>) -> Self {
        Self { words }
    }

    pub fn take_word(&mut self, word: &String) -> Option<String> {
        Some(
            self.words
                .remove(self.words.iter().position(|w| w == word)?),
        )
    }

    pub fn give_word(&mut self, word: String) {
        self.words.push(word)
    }

    pub fn words(&self) -> Vec<String> {
        self.words.clone()
    }
}
