use crate::behavior::{Behavior, BehaviorType};
use std::error::Error;

pub struct ExtractUrls {
    pub json_path: String,
}

impl ExtractUrls {
    pub fn new(json_path: String) -> Self {
        Self { json_path }
    }
}

impl Behavior for ExtractUrls {
    fn behavior_type(&self) -> BehaviorType {
        BehaviorType::Url
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        // TODO: open self.json_path, parse JSON, extract each URL and print it
        println!("ExtractUrls: would process JSON at \"{}\"...", self.json_path);
        Ok(())
    }
}
