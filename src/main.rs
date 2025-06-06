use clap::{Parser, Subcommand};
use std::error::Error;

mod common;
use common::OutputFormat;

mod behavior;
use behavior::{Behavior, BehaviorType};

mod behaviors;
use behaviors::*;

/// CLI entrypoint—Clap automatically provides `--help`.
#[derive(Parser)]
#[command(name = "telegram-analyzer")]
#[command(about = "A CLI tool to analyze Telegram JSON, file metadata, etc.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// All the subcommands correspond to individual Behaviors (or aggregators).
#[derive(Subcommand)]
enum Commands {
    /// Extract URLs from JSON and print each one
    ExtractUrls {
        /// Path to input JSON
        #[arg(short, long)]
        json: String,
    },

    /// Extract all URLs and print each URL with the number of times it appeared
    CountUrls {
        /// Path to the file to scan for URLs.
        #[arg(short, long)]
        input: String,

        /// (Optional) Where to save the URL,count pairs. If omitted, prints to stdout.
        #[arg(short, long)]
        output: Option<String>,

        /// (Optional) Output format: “stdout” (default), “txt”, or “csv”.
        #[arg(short, long, value_enum)]
        format: Option<OutputFormat>,
    },

    /// From JSON, count how many events occurred in each 30-minute slot (e.g. 13:30–14:00)
    CountTimeSlots {
        /// Path to the file to scan for URLs.
        #[arg(short, long)]
        input: String,

        /// (Optional) Where to save the slot,count pairs. If omitted, prints to stdout.
        #[arg(short, long)]
        output: Option<String>,

        /// (Optional) Output format: “stdout” (default), “txt”, or “csv”.
        #[arg(short, long, value_enum)]
        format: Option<OutputFormat>,
    },

    /// From JSON, count how many events occurred each day
    CountDaily {
        /// Path to input JSON
        #[arg(short, long)]
        json: String,
    },

    /// From a folder, get all file extensions and print a list of occurrences per extension
    ListExtensions {
        /// Path to folder
        #[arg(short, long)]
        folder: String,
    },

    /// From a single file, get metadata (name, format, size) and print it
    FileMetadata {
        /// Path to file
        #[arg(short, long)]
        file: String,
    },

    /// From JSON, get all interactions of each user and print id, num_messages, media & links shared, unique links, name/username changes, last_seen
    UserInteractions {
        /// Path to input JSON
        #[arg(short, long)]
        json: String,
    },

    /// From JSON, get total number of messages sent and number of distinct users
    MessageStats {
        /// Path to input JSON
        #[arg(short, long)]
        json: String,
    },

    /// From JSON, output which users shared which media files and links & how many times each link was shared
    Diffusion {
        /// Path to input JSON
        #[arg(short, long)]
        json: String,
    },

    /// From JSON, get who sent which link and who sent which media
    Shares {
        /// Path to input JSON
        #[arg(short, long)]
        json: String,
    },

    /// From JSON, get number of words, characters, sentences, paragraphs, reading_time, word_counter, average word length, num messages with media
    TextStats {
        /// Path to input JSON
        #[arg(short, long)]
        json: String,
    },

    /// Run *all* analyses. Requires:
    ///   --json <path>  (for JSON-based behaviors)
    ///   --folder <path> (for folder-based behaviors)
    ///   --file <path>  (for file-based behaviors)
    All {
        /// JSON file path
        #[arg(short, long)]
        json: String,
        /// Folder path
        #[arg(short, long)]
        folder: String,
        /// Single file path
        #[arg(short, long)]
        file: String,
    },

    /// Run *all* behaviors of a specific BehaviorType.
    /// You must pass (--json / --folder / --file) for those behaviors that need it.
    Group {
        /// BehaviorType to run (ValueEnum lets Clap validate it)
        #[arg(value_enum)]
        behavior_type: BehaviorType,

        /// (Optional) path to JSON (for JSON-based behaviors)
        #[arg(long)]
        json: Option<String>,

        /// (Optional) path to folder (for folder-based behaviors)
        #[arg(long)]
        folder: Option<String>,

        /// (Optional) path to file (for file-metadata)
        #[arg(long)]
        file: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ExtractUrls { json } => {
            let b = ExtractUrls::new(json);
            b.run()?;
        }

        Commands::CountUrls { input, output, format } => {
            let fmt = format.unwrap_or(OutputFormat::Stdout);
            // If user requested Txt or Csv, output folder must be provided
            if (fmt != OutputFormat::Stdout) && output.is_none() {
                return Err("Error: --format not ‘stdout’ requires --output <folder>".into());
            }
            println!("→ Running CountUrls on file: {}", input);
            let b = CountUrls::new(input, output, Some(fmt));
            b.run()?;
        }

        Commands::CountTimeSlots { input, output, format } => {
            let fmt = format.unwrap_or(OutputFormat::Stdout);
            // If user requested Txt or Csv, output folder must be provided
            if (fmt != OutputFormat::Stdout) && output.is_none() {
                return Err("Error: --format not ‘stdout’ requires --output <folder>".into());
            }
            println!("→ Running CountTimeSlots on JSON: {}", input);
            let b = CountTimeSlots::new(input, output, Some(fmt));
            b.run()?;
        }

        Commands::CountDaily { json } => {
            let b = CountDaily::new(json);
            b.run()?;
        }

        Commands::ListExtensions { folder } => {
            let b = ListExtensions::new(folder);
            b.run()?;
        }

        Commands::FileMetadata { file } => {
            let b = FileMetadata::new(file);
            b.run()?;
        }

        Commands::UserInteractions { json } => {
            let b = UserInteractions::new(json);
            b.run()?;
        }

        Commands::MessageStats { json } => {
            let b = MessageStats::new(json);
            b.run()?;
        }

        Commands::Diffusion { json } => {
            let b = Diffusion::new(json);
            b.run()?;
        }

        Commands::Shares { json } => {
            let b = Shares::new(json);
            b.run()?;
        }

        Commands::TextStats { json } => {
            let b = TextStats::new(json);
            b.run()?;
        }

        Commands::All { json, folder, file } => {
            // “All” implicitly uses stdout for CountUrls/CountTimeSlots,
            // or their default behavior if output not needed.
            let behaviors: Vec<Box<dyn Behavior>> = vec![
                Box::new(ExtractUrls::new(json.clone())),
                Box::new(CountUrls::new(json.clone(), None, Some(OutputFormat::Stdout))),
                Box::new(CountTimeSlots::new(json.clone(), None, Some(OutputFormat::Stdout))),
                Box::new(CountDaily::new(json.clone())),
                Box::new(ListExtensions::new(folder.clone())),
                Box::new(FileMetadata::new(file.clone())),
                Box::new(UserInteractions::new(json.clone())),
                Box::new(MessageStats::new(json.clone())),
                Box::new(Diffusion::new(json.clone())),
                Box::new(Shares::new(json.clone())),
                Box::new(TextStats::new(json.clone())),
            ];
            for b in behaviors {
                b.run()?;
            }
        }

        Commands::Group {
            behavior_type,
            json,
            folder,
            file,
        } => {
            // Build all behaviors, but only run those matching behavior_type.
            // For CountUrls/CountTimeSlots in Group, default to stdout.
            let mut behaviors: Vec<Box<dyn Behavior>> = Vec::new();
            behaviors.push(Box::new(ExtractUrls::new(json.clone().unwrap_or_default())));
            behaviors.push(Box::new(CountUrls::new(
                json.clone().unwrap_or_default(),
                None,
                Some(OutputFormat::Stdout),
            )));
            behaviors.push(Box::new(CountTimeSlots::new(
                json.clone().unwrap_or_default(),
                None,
                Some(OutputFormat::Stdout),
            )));
            behaviors.push(Box::new(CountDaily::new(json.clone().unwrap_or_default())));
            behaviors.push(Box::new(ListExtensions::new(folder.clone().unwrap_or_default())));
            behaviors.push(Box::new(FileMetadata::new(file.clone().unwrap_or_default())));
            behaviors.push(Box::new(UserInteractions::new(json.clone().unwrap_or_default())));
            behaviors.push(Box::new(MessageStats::new(json.clone().unwrap_or_default())));
            behaviors.push(Box::new(Diffusion::new(json.clone().unwrap_or_default())));
            behaviors.push(Box::new(Shares::new(json.clone().unwrap_or_default())));
            behaviors.push(Box::new(TextStats::new(json.clone().unwrap_or_default())));

            for b in behaviors {
                if b.behavior_type() == behavior_type {
                    b.run()?;
                }
            }
        }
    }

    Ok(())
}
