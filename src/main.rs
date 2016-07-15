extern crate getopts;

use std::io::prelude::*;
use std::cmp;
use getopts::Options;

const PROGRAM_NAME: &'static str = "rsay";
const COW: &'static str = include_str!("cow.txt");
const DEFAULT_LINE_WIDTH: usize = 40;

fn print_usage (opts: Options) {
    let brief = format!("Usage: {} [-OPTIONS] [ARG...]", PROGRAM_NAME);
    print!("{}", opts.usage(&brief));
}

fn parse_numeric(value: String, default: usize) -> usize {
    match value.parse::<usize>() {
        Ok(n) => { n },
        Err(_) => { default },
    }
}

fn chunk_args (args: Vec<String>, max_size: usize) -> Vec<String> {
    let mut lines = vec![]; // we should declare a size for this to save on allocation...
    let remainder: String = args.iter()
        .fold(String::new(), |mut acc, arg| {
            if !acc.is_empty() {
                if (arg.chars().count() + 1) + acc.chars().count() <= max_size {
                    lines.push(acc.clone() + " " + arg);
                    acc.clear();
                    return acc;
                } else {
                    lines.push(acc.clone());
                    acc.clear();
                }
            }

            for c in arg.chars() {
                acc.push(c);
                if acc.chars().count() == max_size {
                    lines.push(acc.clone());
                    acc.clear();
                }
            }

            acc
        });

    if !remainder.is_empty() {
        lines.push(remainder);
    }

    lines
}

fn multi_line (lines: Vec<String>, width: usize) -> String {
    let total_length = lines.len() - 1;

    let formatted_lines = lines
        .iter()
        .enumerate()
        .map(|(idx, line)| {
            let current_length = line.clone().chars().count();
            let padding: String = (0..width - current_length).map(|_| ' ').collect();
            let (start, end) = match idx {
                0 => ('/', '\\'),
                _ if idx == total_length => ('\\', '/'),
                _ => ('|', '|'),
            };

            format!("{} {}{} {}\n", start, line, padding, end)
        });

    formatted_lines.collect()
}

fn say (args: Vec<String>, desired_width: usize) -> String {
    let chunks = chunk_args(args, desired_width);
    let largest_str = chunks.iter().map(|x| x.chars().count()).max();
    let width = match largest_str {
        Some(x) => { cmp::min(desired_width, x) },
        _ => { desired_width }
    };
    let formatted = match chunks.len() {
        1 => format!("< {} >\n", chunks.join(" ")),
        _ => multi_line(chunks, width),
    };
    let top_border = (0..width + 2).map(|_| "_").collect::<String>();
    let bottom_border = (0..width + 2).map(|_| "-").collect::<String>();

    format!(" {}\n{} {}", top_border, formatted, bottom_border)
}

fn main () {
    let args: Vec<String> = std::env::args()
        .skip(1)
        .collect();
    let mut opts = Options::new();

    opts.optflag("h", "help", "Print this help menu");
    opts.optmulti("W", "width", "Width of output", "50");

    let matches = match opts.parse(&args) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(opts);
        return;
    }

    let width = match matches.opt_str("W") {
        None => { DEFAULT_LINE_WIDTH },
        Some(w) => { parse_numeric(w, DEFAULT_LINE_WIDTH) }
    };

    let input = if !matches.free.is_empty() {
        matches.free
    } else {
        print_usage(opts);
        return;
    };

    print!("{}\n{}", say(input, width), COW);
}

#[cfg(test)]
#[test]
fn test_chunk_args_padding () {
    let phrase = ["broken", "big", "bar"].iter().map(|&x| x.into()).collect();
    let result = chunk_args(phrase, 5);
    assert_eq!(vec!["broke".to_string(), "n big".into(), "bar".into()], result);
}

#[test]
fn test_say_multi_line () {
    let args = ["broke", "n big", "bar"].iter().map(|&x| x.into()).collect();
    let result = say(args, 5);
    let expected: String = r" _______
/ broke \
| n big |
\ bar   /
 -------".into();

    assert_eq!(expected, result);
}

#[test]
fn test_say_multi_line_wide () {
    let phrase = "aggregate rotor hat".split(" ").map(|x| x.into()).collect();
    let result = chunk_args(phrase, 10);
    assert_eq!(vec!["aggregate", "rotor hat"], result);
}

#[test]
fn test_say_single_line () {
    let args = ["foo bar baz"].iter().map(|&x| x.into()).collect();
    let result = say(args, 40);
    let expected: String = r" _____________
< foo bar baz >
 -------------".into();

    assert_eq!(expected, result);
}
