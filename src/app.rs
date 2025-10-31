use std::{io::Read, os::unix::fs::MetadataExt, path::PathBuf};

use clap::Parser;

/// Prints newline, word and byte counts for a FILE
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct App {
    /// Prints the byte count.
    #[arg(short, long = "bytes")]
    count: bool,

    /// Prints the newline counts
    #[arg(short, long)]
    lines: bool,

    /// Prints the word counts
    #[arg(short, long)]
    words: bool,

    /// Prints the character counts
    #[arg(short = 'm', long)]
    chars: bool,

    file: Option<PathBuf>,
}

impl App {
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let this = Self::parse();

        // If no flags were specified, we treat it as if
        // the user wants the default behavior (lines, words, and bytes).
        let no_flags_specified = !this.count && !this.lines && !this.words && !this.chars;

        // Determine what we actually need to show based on flags or defaults
        let show_lines = this.lines || no_flags_specified;
        let show_chars = this.chars;
        let show_words = this.words || no_flags_specified;
        let show_bytes = this.count || no_flags_specified;

        match &this.file {
            Some(filepath) => {
                this.process_file(filepath, show_lines, show_words, show_bytes, show_chars)?;
            }
            None => {
                this.process_stdin(show_lines, show_words, show_bytes, show_chars)?;
            }
        }

        Ok(())
    }

    fn process_file(
        &self,
        filepath: &PathBuf,
        show_lines: bool,
        show_words: bool,
        show_bytes: bool,
        show_chars: bool,
    ) -> Result<(), std::io::Error> {
        let metadata = match std::fs::metadata(filepath) {
            Ok(md) => md,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    eprintln!("ccwc: {} No such file or directory", filepath.display());
                    return Err(e);
                }
                _ => {
                    eprintln!("ccwc: {e}");
                    return Err(e);
                }
            },
        };

        // Read file contents only if we need to count lines, words, or chars
        let contents = if show_lines || show_words || show_chars {
            match std::fs::read_to_string(filepath) {
                Ok(contents) => Some(contents),
                Err(e) => {
                    eprintln!("ccwc: {} {} ", filepath.display(), e);
                    return Err(e);
                }
            }
        } else {
            None
        };

        // Build the output based on what is needed to be shown.
        let mut output = String::new();

        if show_lines && let Some(ref contents) = contents {
            output.push_str(&format!("{} ", contents.lines().count()));
        }

        if show_words && let Some(ref contents) = contents {
            output.push_str(&format!("{} ", contents.split_whitespace().count()));
        }

        if show_chars && let Some(ref contents) = contents {
            output.push_str(&format!("{} ", contents.chars().count()));
        }

        if show_bytes {
            output.push_str(&format!("{} ", metadata.size()));
        }

        // Append file name at the end
        output.push_str(&format!("{}", filepath.display()));

        println!("{}", output);

        Ok(())
    }

    fn process_stdin(
        &self,
        show_lines: bool,
        show_words: bool,
        show_bytes: bool,
        show_chars: bool,
    ) -> Result<(), std::io::Error> {
        let mut contents = String::new();
        std::io::stdin().read_to_string(&mut contents)?;

        let mut output = String::new();

        if show_lines {
            output.push_str(&format!("{} ", contents.lines().count()));
        }

        if show_words {
            output.push_str(&format!("{} ", contents.split_whitespace().count()));
        }

        if show_chars {
            output.push_str(&format!("{} ", contents.chars().count()));
        }

        if show_bytes {
            // For stdin, we count bytes from the string we read
            // contents.len() gives us the byte count
            output.push_str(&format!("{} ", contents.len()));
        }

        // No filename at the end since stdin
        println!("{}", output.trim_end());
        Ok(())
    }
}
