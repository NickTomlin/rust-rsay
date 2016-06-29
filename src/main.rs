use std::io::prelude::*;
use std::cmp;

const GNU: &'static str = include_str!("gnu.txt");
// todo: accept a wraplength param (-W)
const LINE_LENGTH: usize = 40;

fn main () {
    let args: Vec<String> = std::env::args()
        .skip(1)
        .collect();

    if args.len() == 0 {
        let _ = writeln!(std::io::stderr(), "Usage: gnusay PHRASE");
        std::process::exit(1);
    }

    let formatted = format_line(args.join(" "));
    println!("{}", formatted);
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
        // todo: this is gross
        // we should figure out a way to just use the padding on formatting
        while size < chunk_size {
            chunk.push(' ');
            size = size + 1;
        }

        chunks.push(chunk);
    }

    chunks
}

fn multi_line (phrase: String) -> String {
    let lines = chunk_string(phrase, LINE_LENGTH);
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

pub fn format_line (phrase: String) -> String {
    let number_of_chars = phrase.chars().count();
    let number_of_lines = number_of_chars / LINE_LENGTH;
    let border_length = cmp::min(LINE_LENGTH, number_of_chars);
    let border = (0..border_length + 2).map(|_| "-").collect::<String>();

    let formatted = match number_of_lines {
        0 => single_line(phrase),
        _ => multi_line(phrase),
    };

    format!(" {border}\n{} {border}\n{}", formatted, GNU, border = border)
}

#[cfg(test)]
#[test]
fn test_format_line () {
    let mut x: &str = "";
    format_line("Foo".to_string(), "Bar".to_string(), x);
    assert_eq!("Bar Foo", x);
}
