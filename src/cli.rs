use crate::options::*;
use crate::print::print;

const HELP_STR: &'static str = "Usage: rolcat [<option>] <file>...\n
Available Options:
    -h, --help      Show this message
    -v, --version   Print version
    -i --invert     Colour background instead of foreground
    --seed          Seed from which colour starts
                    It must be a hue value
                    Usage: rolcat --seed <int> <file>...
                    Default: <random>
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

pub fn parse(args: &Vec<String>) -> Option<(Options, usize)> {
    let mut i = 1;
    let mut options = Options::default();

    let len = args.len();

    let mut h = options.seed();

    loop {
        if i == len {
            break;
        }

        match args[i].as_str() {
            "-h" | "--help" => {
                let line_shift = options.line_shift();
                let char_shift = options.char_shift();
                for line in HELP_STR.split("\n") {
                    print(line, h, char_shift, options.invert());
                    h += line_shift;
                }
                return None;
            }

            "-v" | "--version" => {
                let version = option_env!("CARGO_PKG_VERSION").unwrap_or("<unknown>");
                print(
                    &format!("rolcat version: {}", version),
                    h,
                    options.char_shift(),
                    options.invert(),
                );
                return None;
            }

            "-i" | "--invert" => {
                options.set_invert(true);
            }

            "--seed" => {
                i += 1;
                if i == len {
                    eprintln!("A valid hue must be supplied");
                    return None;
                } else {
                    match args[i].parse::<u16>() {
                        Ok(seed) => {
                            options.set_seed(seed);
                            h = options.seed();
                        }
                        Err(e) => {
                            eprintln!("Valid Integer must be supplied\n\n{}", e);
                            return None;
                        }
                    }
                }
            }

            "-d" | "-dir" => {
                i += 1;
                if i == len {
                    eprintln!("A valid direction must be supplied");
                    return None;
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
                        dir => {
                            eprintln!("Invalid direction {}\n\t{}", dir, HELP_STR);
                            return None;
                        }
                    }
                }
            }

            "-s" | "-shift" => {
                i += 1;
                if i == len {
                    eprintln!("A integer must be supplied");
                    return None;
                } else {
                    match args[i].parse::<i16>() {
                        Ok(shift) => options.set_shift(shift),
                        Err(e) => {
                            eprintln!("Valid Integer must be supplied\n\n{}", e);
                            return None;
                        }
                    }
                }
            }
            _ => break,
        }

        i += 1;
    }

    if i == len {
        eprintln!("A text file must be supplied");
        return None;
    }

    Some((options, i))
}
