use crate::behavior::{Behavior, BehaviorType};
use std::error::Error;

pub struct Shares {
    pub json_path: String,
}

impl Shares {
    pub fn new(json_path: String) -> Self {
        Self { json_path }
    }
}

impl Behavior for Shares {
    fn behavior_type(&self) -> BehaviorType {
        BehaviorType::Shares
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        // TODO: open self.json_path, for each message extract who sent which link & who sent which media
        println!("Shares: would list user→media and user→link in \"{}\"...", self.json_path);
        Ok(())
    }
}
