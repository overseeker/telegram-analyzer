use crate::behavior::{Behavior, BehaviorType};
use std::error::Error;

pub struct MessageStats {
    pub json_path: String,
}

impl MessageStats {
    pub fn new(json_path: String) -> Self {
        Self { json_path }
    }
}

impl Behavior for MessageStats {
    fn behavior_type(&self) -> BehaviorType {
        BehaviorType::MessageStats
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        // TODO: open self.json_path, count "number of messages sent" and
        //       "number of distinct users who sent messages"
        println!("MessageStats: would compute message & distinct-user counts in \"{}\"...", self.json_path);
        Ok(())
    }
}
