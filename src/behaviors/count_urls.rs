// src/behaviors/count_urls.rs

use clap::ValueEnum;
use regex::Regex;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use crate::behavior::{Behavior, BehaviorType};

/// Choose whether to output “plain” (count<TAB>URL per line) or “csv” ("count","URL").
#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Plain,
    Csv,
}

/// CountUrls behavior:
///
///   - Reads `input_path` line by line, finds every substring matching `https?://…`.
///   - Tallies how many times each URL appears.
///   - Writes lines of `count<TAB>URL` (plain) or `"count","URL"` (csv) into `output_path` (or stdout),
///     ordered by descending count.
pub struct CountUrls {
    pub input_path: String,
    pub output_path: Option<String>,
    pub format: OutputFormat,
}

impl CountUrls {
    /// Constructor. If `format` is `None`, defaults to `OutputFormat::Plain`.
    pub fn new(
        input_path: String,
        output_path: Option<String>,
        format: Option<OutputFormat>,
    ) -> Self {
        let fmt = format.unwrap_or(OutputFormat::Plain);
        CountUrls {
            input_path,
            output_path,
            format: fmt,
        }
    }
}

impl Behavior for CountUrls {
    fn behavior_type(&self) -> BehaviorType {
        BehaviorType::UrlCount
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        // 1) Validate that the input file exists
        if !Path::new(&self.input_path).is_file() {
            return Err(format!("Input file not found: {}", &self.input_path).into());
        }

        // 2) Open input_path for reading
        let file = File::open(&self.input_path)?;
        let reader = BufReader::new(file);

        // 3) Prepare a regex that matches http:// or https:// URLs
        let url_regex = Regex::new(r"https?://[A-Za-z0-9\./\?=\+_\-]+")?;

        // 4) Count occurrences in a BTreeMap
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for line_res in reader.lines() {
            let line = line_res?;
            for mat in url_regex.find_iter(&line) {
                let url = mat.as_str().to_string();
                *counts.entry(url).or_default() += 1;
            }
        }

        // 5) Convert to a Vec and sort by count descending (tie-breaker: URL lex)
        let mut entries: Vec<(&String, &usize)> = counts.iter().collect();
        entries.sort_by(|a, b| {
            b.1.cmp(a.1).then_with(|| a.0.cmp(b.0))
        });

        let unique_count = entries.len();

        // 6) Write results to file or stdout
        match &self.output_path {
            Some(path) => {
                // Ensure parent directory exists
                if let Some(parent) = Path::new(path).parent() {
                    fs::create_dir_all(parent)?;
                }
                let mut f = File::create(path)?;
                match self.format {
                    OutputFormat::Plain => {
                        for (url, cnt) in &entries {
                            writeln!(f, "{}\t{}", cnt, url)?;
                        }
                    }
                    OutputFormat::Csv => {
                        for (url, cnt) in &entries {
                            writeln!(f, "\"{}\",\"{}\"", cnt, url)?;
                        }
                    }
                }
                println!(
                    "→ CountUrls: Found {} unique URLs, results saved to {}",
                    unique_count, path
                );
            }
            None => {
                // Print to stdout
                match self.format {
                    OutputFormat::Plain => {
                        for (url, cnt) in &entries {
                            println!("{}\t{}", cnt, url);
                        }
                    }
                    OutputFormat::Csv => {
                        for (url, cnt) in &entries {
                            println!("\"{}\",\"{}\"", cnt, url);
                        }
                    }
                }
                println!(
                    "→ CountUrls: Found {} unique URLs (printed to stdout)",
                    unique_count
                );
            }
        }

        Ok(())
    }
}
