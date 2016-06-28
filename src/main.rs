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

    // let phrase = phrases.join("");
    // let mut combined = String::new();
    // format_line(phrase, &mut &combined);

    // // todo, format this to match the length of the longest string
    // println!(" _______");
    // print!("< {} >", combined);
    // println!(" _______");
    // println!("{}",  gnu);
}

fn multiline (phrase: String) -> String {
    String::from("hey")
}

pub fn format_line (phrases: Vec<String>) -> String {
    let phrase = phrases.join(" ");
    let lines = phrase.chars().count() / LINE_LENGTH;
    let border = (0..LINE_LENGTH).map(|_| "_").collect::<String>();

    let formatted = match lines {
        0 =>  String::from("< ") + &phrase + " >",
        _ => multiline(phrase)
    };

    // we could "takewhile" the string has stuff in it, in increments of 40 characters
    format!("{border}\n{formatted}\n{border}\n{gnu}", formatted = formatted, gnu = GNU, border = border)
}

#[cfg(test)]
#[test]
fn test_format_line () {
    let mut x: &str = "";
    format_line("Foo".to_string(), "Bar".to_string(), x);
    assert_eq!("Bar Foo", x);
}
