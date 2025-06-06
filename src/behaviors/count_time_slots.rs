// src/behaviors/count_time_slots.rs

use crate::behavior::{Behavior, BehaviorType};
use crate::common::OutputFormat; // shared enum: Stdout, Txt, Csv
use chrono::{DateTime, Timelike, Utc};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

/// CountTimeSlots behavior:
///
///   - Reads `input_path`, expecting a top-level JSON array of objects,
///     each object with a `"date"` key whose value is an ISO 8601 timestamp.
///   - Parses each timestamp, buckets it into a 30-minute slot
///     (e.g. "13:00-13:30", "13:30-14:00"), and counts how many messages
///     fall into each slot.
///   - If `format` is `Stdout`, prints slot<TAB>count lines to stdout.
///   - If `format` is `Txt`, writes a file `<output_dir>/messages_per_slot.txt`
///     with tab-separated output + a summary line.
///   - If `format` is `Csv`, writes a file `<output_dir>/messages_per_slot.csv`
///     with a CSV header + quoted pairs + a summary line.
pub struct CountTimeSlots {
    pub input_path: String,
    pub output_dir: Option<String>,
    pub format: OutputFormat,
}

impl CountTimeSlots {
    /// Constructor. If `format` is `None`, defaults to `OutputFormat::Stdout`.
    pub fn new(
        input_path: String,
        output_dir: Option<String>,
        format: Option<OutputFormat>,
    ) -> Self {
        let fmt = format.unwrap_or(OutputFormat::Stdout);
        CountTimeSlots {
            input_path,
            output_dir,
            format: fmt,
        }
    }
}

impl Behavior for CountTimeSlots {
    fn behavior_type(&self) -> BehaviorType {
        BehaviorType::TimeSlot
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        // 1) Validate that the input file exists
        let path = Path::new(&self.input_path);
        if !path.is_file() {
            return Err(format!("Input file not found: {}", &self.input_path).into());
        }

        // 2) Open and parse JSON as a top-level array
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let data: Value = serde_json::from_reader(reader)?;
        let messages = data
            .as_array()
            .ok_or("Expected top-level JSON array")?;

        // 3) Count each 30-minute slot
        let mut slot_counts: HashMap<String, usize> = HashMap::new();
        let mut total_processed = 0;
        for element in messages {
            let date_str = match element.get("date").and_then(Value::as_str) {
                Some(s) => s,
                None => continue,
            };

            // Parse as RFC 3339
            let dt: DateTime<Utc> = match DateTime::parse_from_rfc3339(date_str) {
                Ok(parsed) => parsed.with_timezone(&Utc),
                Err(_) => continue,
            };

            total_processed += 1;
            let hour = dt.hour();
            let minute = dt.minute();

            let slot = if minute < 30 {
                format!("{:02}:00-{:02}:30", hour, hour)
            } else {
                let next_hour = (hour + 1) % 24;
                format!("{:02}:30-{:02}:00", hour, next_hour)
            };

            *slot_counts.entry(slot).or_default() += 1;
        }

        // 4) Prepare all 48 half-hour slots in chronological order
        let mut all_slots = Vec::with_capacity(48);
        for h in 0..24 {
            let s1 = format!("{:02}:00-{:02}:30", h, h);
            let s2 = format!("{:02}:30-{:02}:00", h, (h + 1) % 24);
            all_slots.push(s1);
            all_slots.push(s2);
        }

        // 5) Branch on format choice
        match self.format {
            OutputFormat::Stdout => {
                // Print to stdout (tab-separated)
                for slot in &all_slots {
                    let count = slot_counts.get(slot).copied().unwrap_or(0);
                    println!("{}\t{}", slot, count);
                }
                println!("Analyzed {} messages", total_processed);
            }
            OutputFormat::Txt => {
                // Must have an output_dir to write a .txt file
                let dir = self.output_dir.as_deref().unwrap_or(".");
                fs::create_dir_all(dir)?;
                let mut output_path = PathBuf::from(dir);
                output_path.push("messages_per_slot.txt");

                let mut f = File::create(&output_path)?;
                for slot in &all_slots {
                    let count = slot_counts.get(slot).copied().unwrap_or(0);
                    writeln!(f, "{}\t{}", slot, count)?;
                }
                writeln!(f, "Analyzed {} messages", total_processed)?;

                println!(
                    "→ CountTimeSlots: Processed {} messages, results saved to {}",
                    total_processed,
                    output_path.display()
                );
            }
            OutputFormat::Csv => {
                // Must have an output_dir to write a .csv file
                let dir = self.output_dir.as_deref().unwrap_or(".");
                fs::create_dir_all(dir)?;
                let mut output_path = PathBuf::from(dir);
                output_path.push("messages_per_slot.csv");

                let mut f = File::create(&output_path)?;
                writeln!(f, "\"Slot\",\"Count\"")?;
                for slot in &all_slots {
                    let count = slot_counts.get(slot).copied().unwrap_or(0);
                    writeln!(f, "\"{}\",\"{}\"", slot, count)?;
                }
                writeln!(f, "\"Analyzed {} messages\",\"\"", total_processed)?;

                println!(
                    "→ CountTimeSlots: Processed {} messages, results saved to {}",
                    total_processed,
                    output_path.display()
                );
            }
        }

        Ok(())
    }
}
