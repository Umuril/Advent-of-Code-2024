use crate::template::{all_days, aoc_cli, Day};
use std::process;

pub fn handle(day: Option<Day>, all: bool) {
    if aoc_cli::check().is_err() {
        eprintln!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
        process::exit(1);
    }

    if all {
        for day in all_days() {
            if let Err(e) = aoc_cli::download(day) {
                eprintln!("failed to call aoc-cli: {e}");
                process::exit(1);
            }
        }
    } else if let Some(day) = day {
        if let Err(e) = aoc_cli::download(day) {
            eprintln!("failed to call aoc-cli: {e}");
            process::exit(1);
        };
    } else {
        eprintln!("Choose a day or --all.");
        process::exit(1);
    }
}
