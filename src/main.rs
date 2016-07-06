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

fn chunk_args (args: Vec<String>, chunk_size: usize) -> Vec<String> {
    let mut chunks = Vec::new();

    for arg in args {
        let mut x = 0;
        let mut chunk = String::with_capacity(chunk_size);
        for char in arg.chars() {
            if x < chunk_size {
                chunk.push(char);
                x = x + 1;
            }
            if x == chunk_size {
                chunks.push(chunk.clone());
                chunk.clear();
                x = 0;
            }
        }
        if !chunk.is_empty() {
            while chunk.len() < chunk_size {
                chunk.push(' ');
            }
            chunks.push(chunk.clone());
        }
    }

    chunks
}

fn multi_line (args: Vec<String>, width: usize) -> String {
    let lines = chunk_args(args, width);
    let total_length = lines.len() - 1;

    let formatted_lines = lines
        .iter()
        .enumerate()
        .map(|(idx, line)| {
            let (start, end) = match idx {
                0 => ('/', '\\'),
                _ if idx == total_length => ('\\', '/'),
                _ => ('|', '|'),
            };

            format!("{} {} {}\n", start, line, end)
        });

    formatted_lines.collect()
}

fn single_line (phrase: Vec<String>) -> String {
    format!("< {} >\n", phrase.join(" "))
}

fn say (args: Vec<String>, width: usize) -> String {
    let phrase = args.join(" ");
    let number_of_chars = phrase.chars().count();
    let number_of_lines = number_of_chars / width;
    let border_length = cmp::min(width, number_of_chars);
    let border = (0..border_length + 2).map(|_| "-").collect::<String>();

    let formatted = match number_of_lines {
        0 => single_line(args),
        _ => multi_line(args, width),
    };

    format!(" {border}\n{} {border}", formatted, border = border)
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

    println!("{}\n{}", say(input, width), COW);
}

#[cfg(test)]
#[test]
fn test_chunk_args_padding () {
    let phrase = ["fooooo", "bar", "baz"].iter().map(|&x| x.into()).collect();
    let result = chunk_args(phrase, 5);
    assert_eq!(vec!["foooo".to_string(), "o    ".into(), "bar  ".into(), "baz  ".into()], result);
}

#[test]
fn test_say_multi_line () {
    let args = ["fooooo", "bar", "baz"].iter().map(|&x| x.into()).collect();
    let result = say(args, 5);
    let expected: String = r" -------
/ foooo \
| o     |
| bar   |
\ baz   /
 -------".into();

    assert_eq!(expected, result);
}

#[test]
fn test_say_single_line () {
    let args = ["foo bar baz"].iter().map(|&x| x.into()).collect();
    let result = say(args, 40);
    let expected: String = r" -------------
< foo bar baz >
 -------------".into();

    assert_eq!(expected, result);
}
