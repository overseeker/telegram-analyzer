use clap::ValueEnum;
use std::error::Error;

/// Every “behavior” (functionality) must implement this trait.
pub trait Behavior {
    /// Returns the kind of behavior, so you can filter/group them.
    fn behavior_type(&self) -> BehaviorType;

    /// Run the analysis/operation.  
    /// (Right now, stubs—all the real work is `// TODO`.)
    fn run(&self) -> Result<(), Box<dyn Error>>;
}

/// Enumerates all behavior‐kinds.  
/// This lets “group” or “all” subcommands pick which behaviors to run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum BehaviorType {
    Url,             // extract-urls
    UrlCount,        // count-urls
    TimeSlot,        // count-time-slots
    Daily,           // count-daily
    Extensions,      // list-extensions
    FileMetadata,    // file-metadata
    UserInteractions,// user-interactions
    MessageStats,    // message-stats
    Diffusion,       // diffusion
    Shares,          // shares
    TextStats,       // text-stats
}
