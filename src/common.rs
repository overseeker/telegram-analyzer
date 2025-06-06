use clap::ValueEnum;

/// Shared OutputFormat for all behaviors:
///   - `Stdout`: print to standard output (ignores `output_dir`),
///   - `Txt`: write a plain‐text file (`*.txt`) in `output_dir`,
///   - `Csv`: write a CSV file (`*.csv`) in `output_dir`.
#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum OutputFormat {
    Stdout,
    Txt,
    Csv,
}