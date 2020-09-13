use std::env;
use std::fs;
use std::io::{self, prelude::*, BufReader};

mod options;
use options::*;

const HELP_STR: &'static str = "Usage: rolcat [<option>] <file>...\n
Available Options:
    -h, --help      Show this message
    -v, --version   Print version
    -d, -dir        Choose the direction for colour shift to occur
                    Usage: rolcat -dir <dir> <file>...
                    Available directions: [tr, t, tl, r, l, br, b, bl] where:
                        t = top,
                        b = bottom,
                        r = right,
                        l = left,
                    Default: bottom right
    -s, -shift      The hue shift per character
                    Usage: rolcat -shift <int> <file>...
                    Default: 2";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    let mut options = Options::default();

    let len = args.len();
    let mut h = rand::random::<f32>() * 360.;

    loop {
        if i == len {
            break;
        }

        match args[i].as_str() {
            "-h" | "--help" => {
                return Ok({
                    let line_shift = options.line_shift();
                    let char_shift = options.char_shift();
                    for line in HELP_STR.split("\n") {
                        print(line, h, char_shift);
                        h += line_shift;
                    }
                })
            }

            "-v" | "--version" => {
                let version = option_env!("CARGO_PKG_VERSION").unwrap_or("<unknown>");
                return Ok({
                    print(
                        &format!("rolcat version: {}", version),
                        h,
                        options.char_shift(),
                    );
                });
            }

            "-d" | "-dir" => {
                i += 1;
                if i == len {
                    return Ok(eprintln!("A valid direction must be supplied"));
                } else {
                    match args[i].as_str() {
                        "tl" => options.set_dir(Direction::TopLeft),
                        "t" => options.set_dir(Direction::Top),
                        "tr" => options.set_dir(Direction::TopRight),
                        "l" => options.set_dir(Direction::Left),
                        "r" => options.set_dir(Direction::Right),
                        "bl" => options.set_dir(Direction::BottomLeft),
                        "b" => options.set_dir(Direction::Bottom),
                        "br" => options.set_dir(Direction::BottomRight),
                        dir => return Ok(eprintln!("Invalid direction {}\n\t{}", dir, HELP_STR)),
                    }
                }
            }

            "-s" | "-shift" => {
                i += 1;
                if i == len {
                    return Ok(eprintln!("A integer must be supplied"));
                } else {
                    match args[i].parse::<i16>() {
                        Ok(shift) => options.set_shift(shift),
                        Err(e) => return Ok(eprintln!("Valid Integer must be supplied\n\n{}", e)),
                    }
                }
            }
            _ => break,
        }

        i += 1;
    }

    if i == len {
        return Ok(eprintln!("A text file must be supplied"));
    }

    let print_name = args.len() > i + 1;

    let char_shift = options.char_shift();
    let line_shift = options.line_shift();

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

fn print(s: &str, h: f32, shift: f32) {
    let mut h = h;
    for chr in s.split("") {
        let (r, g, b) = get_rgb(h);
        print!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, chr);
        h = (360.0 + h + shift) % 360.0;
    }
    println!();
}

fn get_rgb(h: f32) -> (u8, u8, u8) {
    let hv = h / 60.;
    let hi = hv.floor() % 6.;
    let f = (hv - hi) * 255.0;
    let q = 255.0 - f;

    if hi == 0. {
        (255, f as u8, 0)
    } else if hi == 1. {
        (q as u8, 255, 0)
    } else if hi == 2. {
        (0, 255, f as u8)
    } else if hi == 3. {
        (0, q as u8, 255)
    } else if hi == 4. {
        (f as u8, 0, 255)
    } else if hi == 5. {
        (255, 0, q as u8)
    } else {
        println!("---- UNREACHABLE ----");
        println!("h: {}, hv: {}, hi: {}", h, hv, hi);
        unreachable!()
    }
}
