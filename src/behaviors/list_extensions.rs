use crate::behavior::{Behavior, BehaviorType};
use std::error::Error;

pub struct ListExtensions {
    pub folder_path: String,
}

impl ListExtensions {
    pub fn new(folder_path: String) -> Self {
        Self { folder_path }
    }
}

impl Behavior for ListExtensions {
    fn behavior_type(&self) -> BehaviorType {
        BehaviorType::Extensions
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        // TODO: scan self.folder_path recursively, collect file extensions, print extension → count
        println!("ListExtensions: would scan folder \"{}\" for extensions...", self.folder_path);
        Ok(())
    }
}
