use crate::behavior::{Behavior, BehaviorType};
use std::error::Error;

pub struct TextStats {
    pub json_path: String,
}

impl TextStats {
    pub fn new(json_path: String) -> Self {
        Self { json_path }
    }
}

impl Behavior for TextStats {
    fn behavior_type(&self) -> BehaviorType {
        BehaviorType::TextStats
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        // TODO: open self.json_path, compute:
        //   - total words
        //   - total characters
        //   - number of sentences
        //   - number of paragraphs
        //   - estimated reading_time
        //   - word_counter
        //   - average word length
        //   - number of messages containing media
        println!("TextStats: would compute text metrics in \"{}\"...", self.json_path);
        Ok(())
    }
}
