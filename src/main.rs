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

fn chunk_string (phrase: String, chunk_size: usize) -> Vec<String> {
    let mut chunks = vec![];
    let mut chunk = String::new();
    let mut size = 0;

    for char in phrase.chars() {
        if size == chunk_size {
            chunks.push(chunk);
            size = 0;
            chunk = String::new();
        }

        chunk.push(char);
        size = size + 1;
    }

    if !chunk.is_empty() {
        while size < chunk_size {
            chunk.push(' ');
            size = size + 1;
        }

        chunks.push(chunk);
    }

    chunks
}

fn multi_line (phrase: String, width: usize) -> String {
    let lines = chunk_string(phrase, width);
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

    formatted_lines.collect::<String>()
}

fn single_line (phrase: String) -> String {
    format!("< {} >\n", phrase)
}

fn say (phrase: String, width: usize) -> String {
    let number_of_chars = phrase.chars().count();
    let number_of_lines = number_of_chars / width;
    let border_length = cmp::min(width, number_of_chars);
    let border = (0..border_length + 2).map(|_| "-").collect::<String>();

    let formatted = match number_of_lines {
        0 => single_line(phrase),
        _ => multi_line(phrase, width),
    };

    format!(" {border}\n{} {border}\n{}", formatted, COW, border = border)
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
        matches.free.join(" ")
    } else {
        print_usage(opts);
        return;
    };

    println!("{}", say(input, width));
}
