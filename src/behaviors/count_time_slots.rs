use crate::behavior::{Behavior, BehaviorType};
use std::error::Error;

pub struct CountTimeSlots {
    pub json_path: String,
}

impl CountTimeSlots {
    pub fn new(json_path: String) -> Self {
        Self { json_path }
    }
}

impl Behavior for CountTimeSlots {
    fn behavior_type(&self) -> BehaviorType {
        BehaviorType::TimeSlot
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        // TODO: open self.json_path, parse timestamps, bucket them into 30-minute slots, print counts
        println!("CountTimeSlots: would bucket events from \"{}\" into 30-min slots...", self.json_path);
        Ok(())
    }
}
