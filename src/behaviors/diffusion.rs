use crate::behavior::{Behavior, BehaviorType};
use std::error::Error;

pub struct Diffusion {
    pub json_path: String,
}

impl Diffusion {
    pub fn new(json_path: String) -> Self {
        Self { json_path }
    }
}

impl Behavior for Diffusion {
    fn behavior_type(&self) -> BehaviorType {
        BehaviorType::Diffusion
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        // TODO: open self.json_path, for each user, collect:
        //       - which media files they shared
        //       - which links they shared
        //       - how many times each link was shared
        println!("Diffusion: would analyze media+link diffusion in \"{}\"...", self.json_path);
        Ok(())
    }
}
