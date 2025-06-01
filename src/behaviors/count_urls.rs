use crate::behavior::{Behavior, BehaviorType};
use std::error::Error;

pub struct CountUrls {
    pub json_path: String,
}

impl CountUrls {
    pub fn new(json_path: String) -> Self {
        Self { json_path }
    }
}

impl Behavior for CountUrls {
    fn behavior_type(&self) -> BehaviorType {
        BehaviorType::UrlCount
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        // TODO: open self.json_path, parse JSON, count each URL occurrence, then print url → count
        println!("CountUrls: would count URLs in \"{}\"...", self.json_path);
        Ok(())
    }
}
