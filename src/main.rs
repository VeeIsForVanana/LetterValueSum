use clap::Parser;
use core::result::Result;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The argument passed to the letter sum function
    argstring: String,
}

#[derive(Debug, PartialEq)]
enum ArgError {
    NonASCII,
    NonAlphabetic,
    NonLowercase,
}

fn main() {
    let args = Args::parse();
    let result: Result<u32, ArgError> = lettersum(&args.argstring);
    match result {
        Ok(val) => println!("{}", val),
        Err(err) => println!("Error: {:?}", err),
    }
}

fn lettersum(s: &str) -> Result<u32, ArgError> {
    let mut chars = s.chars();
    while let Some(x) = chars.next() {
        if !x.is_ascii() {
            return Err(ArgError::NonASCII);
        } else if !x.is_alphabetic() {
            return Err(ArgError::NonAlphabetic);
        } else if x.is_uppercase() {
            return Err(ArgError::NonLowercase);
        }
    }
    Ok(s.chars().map(|x: char| {x as u32 - 'a' as u32 + 1}).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid() {
        assert_eq!(lettersum("Hello"), Err(ArgError::NonLowercase));
    }

    #[test]
    fn test_none() {
        assert_eq!(lettersum(""), Ok(0));
    }

    #[test]
    fn test_a() {
        assert_eq!(lettersum("a"), Ok(1));
    }

    #[test]
    fn test_z() {
        assert_eq!(lettersum("z"), Ok(26));
    }
}