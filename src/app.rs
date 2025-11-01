use std::{
    fs::File,
    io::{BufReader, Read},
    os::unix::fs::MetadataExt,
    path::PathBuf,
};

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
        let metadata = std::fs::metadata(filepath)?;

        let needs_contents = show_lines || show_words || show_chars;

        let (line_count, word_count, char_count) = if needs_contents {
            let file = File::open(filepath)?;
            let mut reader = BufReader::new(file);

            let mut buffer = Vec::new();

            reader.read_to_end(&mut buffer)?;

            // Count lines by counting newline bytes
            let lines = if show_lines {
                buffer.iter().filter(|&&b| b == b'\n').count()
            } else {
                0
            };

            let (words, chars) = if show_words || show_chars {
                match std::str::from_utf8(&buffer) {
                    Ok(contents) => {
                        let w = if show_words {
                            contents.split_whitespace().count()
                        } else {
                            0
                        };

                        let c = if show_chars {
                            contents.chars().count()
                        } else {
                            0
                        };

                        (w, c)
                    }
                    Err(_) => {
                        // If UTF-8 conversion fails, use this fallback
                        // The real wc would use locale-specific encoding here
                        let w = if show_words {
                            buffer
                                .split(|b| b.is_ascii_whitespace())
                                .filter(|s| !s.is_empty())
                                .count()
                        } else {
                            0
                        };

                        // Treat every byte as one character
                        let c = if show_chars { buffer.len() } else { 0 };

                        (w, c)
                    }
                }
            } else {
                (0, 0)
            };

            (lines, words, chars)
        } else {
            (0, 0, 0)
        };

        // Build the output based on what is needed to be shown.
        let mut output = String::new();

        if show_lines {
            output.push_str(&format!("{} ", line_count));
        }
        if show_words {
            output.push_str(&format!("{} ", word_count));
        }
        if show_chars {
            output.push_str(&format!("{} ", char_count));
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
        let f = std::io::stdin();
        let mut reader = BufReader::new(f);

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let mut output = String::new();

        let lines = if show_lines {
            buffer.iter().filter(|&&b| b == b'\n').count()
        } else {
            0
        };

        let (words, chars) = if show_words || show_chars {
            match std::str::from_utf8(&buffer) {
                Ok(contents) => {
                    let w = if show_words {
                        contents.split_whitespace().count()
                    } else {
                        0
                    };

                    let c = if show_chars {
                        contents.chars().count()
                    } else {
                        0
                    };

                    (w, c)
                }
                Err(_) => {
                    let w = if show_words {
                        buffer
                            .split(|b| b.is_ascii_whitespace())
                            .filter(|s| !s.is_empty())
                            .count()
                    } else {
                        0
                    };

                    let c = if show_chars { buffer.len() } else { 0 };

                    (w, c)
                }
            }
        } else {
            (0, 0)
        };

        if show_lines {
            output.push_str(&format!("{} ", lines));
        }

        if show_words {
            output.push_str(&format!("{} ", words));
        }

        if show_chars {
            output.push_str(&format!("{} ", chars));
        }

        if show_bytes {
            // For stdin, we count bytes from the string we read
            // contents.len() gives us the byte count
            output.push_str(&format!("{} ", buffer.len()));
        }

        // No filename at the end since stdin
        println!("{}", output.trim_end());
        Ok(())
    }
}
