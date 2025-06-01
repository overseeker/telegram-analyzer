use crate::behavior::{Behavior, BehaviorType};
use std::error::Error;

pub struct UserInteractions {
    pub json_path: String,
}

impl UserInteractions {
    pub fn new(json_path: String) -> Self {
        Self { json_path }
    }
}

impl Behavior for UserInteractions {
    fn behavior_type(&self) -> BehaviorType {
        BehaviorType::UserInteractions
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        // TODO: open self.json_path, parse messages, for each user ID collect:
        //  - num_messages
        //  - num_media
        //  - num_links
        //  - num_unique_links
        //  - name‐change count
        //  - username‐change count
        //  - last_seen
        println!("UserInteractions: would analyze interactions in \"{}\"...", self.json_path);
        Ok(())
    }
}
