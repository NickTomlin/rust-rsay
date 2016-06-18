use std::io::prelude::*;

fn main () {
    let phrases: Vec<String> = std::env::args()
        .skip(1)
        .collect();

    if phrases.len() == 0 {
        let _ = writeln!(std::io::stderr(), "Usage: gnusay PHRASE");
        std::process::exit(1);
    }

    let gnu = include_str!("gnu.txt");
    let phrase = phrases.join("");

    // todo, format this to match the length of the longest string
    println!(" _______");
    println!("< {} >", phrase);
    println!(" _______");
    println!("{}",  gnu);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
