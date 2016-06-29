use std::io::prelude::*;

const GNU: &'static str = include_str!("gnu.txt");
// todo: accept a wraplength
const LINE_LENGTH: usize = 40;

fn main () {
    let phrases: Vec<String> = std::env::args()
        .skip(1)
        .collect();

    if phrases.len() == 0 {
        let _ = writeln!(std::io::stderr(), "Usage: gnusay PHRASE");
        std::process::exit(1);
    }

    let p = format_line(phrases);
    println!("{}", p);
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
        chunks.push(chunk);
    }

    chunks
}

fn multi_line (phrase: String) -> String {
    let lines = chunk_string(phrase, LINE_LENGTH);

    lines.join("\n")
}

fn single_line (phrase: String) -> String {
    String::from("< ") + &phrase + " >"
}

pub fn format_line (phrases: Vec<String>) -> String {
    let phrase = phrases.join(" ");
    let number_of_lines = phrase.chars().count() / LINE_LENGTH;
    let border = (0..LINE_LENGTH).map(|_| "_").collect::<String>();

    let formatted = match number_of_lines {
        0 =>  single_line(phrase),
        _ => multi_line(phrase),
    };

    // we could "takewhile" the string has stuff in it, in increments of 40 characters
    format!("{border}\n{}\n{border}\n{}", formatted, GNU, border = border)
}

#[cfg(test)]
#[test]
fn test_format_line () {
    let mut x: &str = "";
    format_line("Foo".to_string(), "Bar".to_string(), x);
    assert_eq!("Bar Foo", x);
}
