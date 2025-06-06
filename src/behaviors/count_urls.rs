// src/behaviors/count_urls.rs

use crate::behavior::{Behavior, BehaviorType};
use crate::common::OutputFormat; // shared enum: Stdout, Txt, Csv
use regex::Regex;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

/// CountUrls behavior:
///
///   - Reads `input_path` line by line, finds every substring matching `https?://…`.
///   - Tallies how many times each URL appears.
///   - If `format` is `Stdout`, prints `count<TAB>URL` lines to stdout.
///   - If `format` is `Txt`, writes `urls_count.txt` in `output_dir` with tab-separated lines + summary.
///   - If `format` is `Csv`, writes `urls_count.csv` in `output_dir` with a CSV header + quoted pairs + summary.
pub struct CountUrls {
    pub input_path: String,
    pub output_dir: Option<String>,
    pub format: OutputFormat,
}

impl CountUrls {
    /// Constructor. If `format` is `None`, defaults to `OutputFormat::Stdout`.
    pub fn new(
        input_path: String,
        output_dir: Option<String>,
        format: Option<OutputFormat>,
    ) -> Self {
        let fmt = format.unwrap_or(OutputFormat::Stdout);
        CountUrls {
            input_path,
            output_dir,
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
        let path = Path::new(&self.input_path);
        if !path.is_file() {
            return Err(format!("Input file not found: {}", &self.input_path).into());
        }

        // 2) Open input_path for reading
        let file = File::open(path)?;
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

        // 5) Convert to a Vec and sort by count descending, then URL lex
        let mut entries: Vec<(&String, &usize)> = counts.iter().collect();
        entries.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

        let unique_count = entries.len();

        // 6) Branch on chosen format
        match self.format {
            OutputFormat::Stdout => {
                // Print to stdout
                for (url, cnt) in &entries {
                    println!("{}\t{}", cnt, url);
                }
                println!("→ CountUrls: Found {} unique URLs (printed to stdout)", unique_count);
            }

            OutputFormat::Txt => {
                // Must have an output_dir to write a .txt file
                let dir = self.output_dir.as_deref().unwrap_or(".");
                fs::create_dir_all(dir)?;
                let mut output_path = PathBuf::from(dir);
                output_path.push("urls_count.txt");

                let mut f = File::create(&output_path)?;
                for (url, cnt) in &entries {
                    writeln!(f, "{}\t{}", cnt, url)?;
                }
                writeln!(f, "→ CountUrls: Found {} unique URLs", unique_count)?;

                println!(
                    "→ CountUrls: Results saved to {}",
                    output_path.display()
                );
            }

            OutputFormat::Csv => {
                // Must have an output_dir to write a .csv file
                let dir = self.output_dir.as_deref().unwrap_or(".");
                fs::create_dir_all(dir)?;
                let mut output_path = PathBuf::from(dir);
                output_path.push("urls_count.csv");

                let mut f = File::create(&output_path)?;
                writeln!(f, "\"Count\",\"URL\"")?;
                for (url, cnt) in &entries {
                    writeln!(f, "\"{}\",\"{}\"", cnt, url)?;
                }
                writeln!(f, "\"→ CountUrls: Found {} unique URLs\",\"\"", unique_count)?;
                
                println!(
                    "→ CountUrls: Results saved to {}",
                    output_path.display()
                );
            }
        }

        Ok(())
    }
}
