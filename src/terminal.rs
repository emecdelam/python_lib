use std::io::{self, BufRead};
use std::process::{Command, exit};
use pyo3::prelude::*;
/// Runs every line in the terminal from a file path
pub fn terminal_from_file(file_path: &str) -> io::Result<()> {
    let file = std::fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        if let Ok(command_line) = line {
            let mut parts = command_line.split_whitespace();
            if let Some(command) = parts.next() {
                let arguments: Vec<_> = parts.collect();
                let status = Command::new(command)
                    .args(arguments)
                    .status();
                match status {
                    Ok(exit_status) => {
                        if !exit_status.success() {
                            eprintln!("Command failed: {:?}", command_line);
                            exit(1);
                        }
                    }
                    Err(err) => {
                        eprintln!("Error running command: {:?}", err);
                        exit(1);
                    }
                }
            }
        }
    }
    Ok(())
}