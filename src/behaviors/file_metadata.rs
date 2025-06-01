use crate::behavior::{Behavior, BehaviorType};
use std::error::Error;

pub struct FileMetadata {
    pub file_path: String,
}

impl FileMetadata {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

impl Behavior for FileMetadata {
    fn behavior_type(&self) -> BehaviorType {
        BehaviorType::FileMetadata
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        // TODO: stat self.file_path, extract name, extension, size, print them
        println!("FileMetadata: would inspect file \"{}\" to get name, format, size...", self.file_path);
        Ok(())
    }
}
