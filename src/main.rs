use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short='c', action)]
    bytes: bool,
    #[clap(short='l', action)]
    lines: bool,
    #[clap(short='w', action)]
    words: bool,
    #[clap(short='m', action)]
    characters: bool,
    path: Option<PathBuf>,
}

struct Counts {
    bytes: u64,
    lines: u64,
    words: u64,
    characters: u64
}

fn main() {
    let args = Args::parse();

    let buf: Box<dyn BufRead>;
    
    match &args.path {
        None => {
            buf = Box::new(BufReader::new(stdin()));
        }
        Some(path) => {
            match File::open(path) {
                Ok(file) => {
                    buf = Box::new(BufReader::new(file));
                }
                Err(error) => {
                    println!("{}", error);
                    exit(1);
                }
            }
        }
    }

    let res = get_counts(buf);
    
    match res {
        Ok(counts) => {
            output(counts, args);
        }
        Err(error) => {
            println!("{}", error);
        }
    }

}

fn output(counts: Counts, args: Args) {
    let all: bool = !args.bytes && !args.lines && !args.words && !args.characters;
    let mut result = String::new();

    if args.bytes || all {
        result.push_str(&counts.bytes.to_string());
        result.push_str(" ");
    }

    if args.lines || all {
        result.push_str(&counts.lines.to_string());
        result.push_str(" ");
    }

    if args.words || all {
        result.push_str(&counts.words.to_string());
        result.push_str(" ");
    }

    if args.characters || all {
        result.push_str(&counts.characters.to_string());
        result.push_str(" ");
    }

    if let Some(path) = &args.path {
        result.push_str(path.to_str().unwrap());
    }

    println!("{}", result);
}

fn get_counts(mut reader: Box<dyn BufRead>) -> Result<Counts, std::io::Error> {
    let mut string = String::new();
    let mut counts: Counts = Counts {
        bytes: 0,
        lines: 0,
        words: 0,
        characters: 0,
    };

    while reader.read_line(&mut string)? > 0 {
        counts.bytes += string.bytes().count() as u64;
        counts.characters += string.chars().count() as u64;
        counts.lines += 1;
        counts.words += string.split_whitespace().count() as u64;
        string.clear();
    }

    Ok(counts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bytes() {
        let bytes = get_counts(Box::new(BufReader::new(File::open(".\\test\\test.txt")
            .unwrap()))).unwrap().bytes;
        assert_eq!(bytes, 342190);
    }

    #[test]
    fn test_get_lines() {
        let lines = get_counts(Box::new(BufReader::new(File::open(".\\test\\test.txt")
            .unwrap()))).unwrap().lines;
        assert_eq!(lines, 7145);
    }

    #[test]
    fn test_get_words() {
        let words = get_counts(Box::new(BufReader::new(File::open(".\\test\\test.txt")
            .unwrap()))).unwrap().words;
        assert_eq!(words, 58164);
    }

    #[test]
    fn test_get_characters() {
        let characters = get_counts(Box::new(BufReader::new(File::open(".\\test\\test.txt")
            .unwrap()))).unwrap().characters;
        assert_eq!(characters, 339292);
    }
}