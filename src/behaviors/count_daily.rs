use crate::behavior::{Behavior, BehaviorType};
use std::error::Error;

pub struct CountDaily {
    pub json_path: String,
}

impl CountDaily {
    pub fn new(json_path: String) -> Self {
        Self { json_path }
    }
}

impl Behavior for CountDaily {
    fn behavior_type(&self) -> BehaviorType {
        BehaviorType::Daily
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        // TODO: open self.json_path, parse timestamps, count events per day, print counts
        println!("CountDaily: would count daily events in \"{}\"...", self.json_path);
        Ok(())
    }
}
