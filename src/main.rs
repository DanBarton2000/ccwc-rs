use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short='c', action)]
    bytes: bool,
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    if args.bytes {
        match get_bytes(&args.path) {
            Ok(bytes) => {
                println!("{} {}", bytes, &args.path.to_str().unwrap());
            }
            Err(e) => {
                println!("{}", e);
            }
        }
        return
    }
}

fn get_bytes(path_buf: &PathBuf) -> Result<u64, std::io::Error> {
    Ok(fs::metadata(&path_buf)?.len())
}

fn get_lines(path_buf: &PathBuf) -> Result<u64, std::io::Error> {
    Ok(0)
}

fn get_words(path_buf: &PathBuf) -> Result<u64, std::io::Error> {
    Ok(0)
}

fn get_characters(path_buf: &PathBuf) -> Result<u64, std::io::Error> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bytes() {
        assert_eq!(get_bytes(&PathBuf::from(".\\test\\test.txt")).unwrap(), 342190);
    }

    #[test]
    fn test_get_bytes_error() {
        assert!(get_bytes(&PathBuf::from(".\\test\\not_a_file.txt")).is_err());
    }

    #[test]
    fn test_get_lines() {
        assert_eq!(get_lines(&PathBuf::from(".\\test\\test.txt")).unwrap(), 7145);
    }

    #[test]
    fn test_get_words() {
        assert_eq!(get_words(&PathBuf::from(".\\test\\test.txt")).unwrap(), 58164);
    }

    #[test]
    fn test_get_characters() {
        assert_eq!(get_characters(&PathBuf::from(".\\test\\test.txt")).unwrap(), 339292);
    }
}