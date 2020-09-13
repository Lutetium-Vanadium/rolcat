use std::env;
use std::fs;
use std::io::{self, prelude::*, BufReader};

mod cli;
mod options;
mod print;

use print::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let (options, i) = match cli::parse(&args) {
        Some(ret) => ret,
        None => return Ok(()),
    };

    let print_name = args.len() > i + 1;

    let char_shift = options.char_shift();
    let line_shift = options.line_shift();

    let mut h = options.seed();

    for file_name in &args[i..] {
        let file = fs::File::open(&file_name)?;
        let reader = BufReader::new(file);
        if print_name {
            println!("> \x1B[38;5;15m{}\x1B[0m", &file_name);
        }

        for line in reader.lines() {
            print(&line?, h, char_shift);
            h = (360.0 + h + line_shift) % 360.0;
        }

        if print_name {
            println!();
        }
    }

    Ok(())
}
