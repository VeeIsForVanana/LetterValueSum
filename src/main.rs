use clap::Parser;
use core::result::Result;
use std::fmt::Display;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The argument passed to the letter sum function
    argstring: String,
}

#[derive(Debug, PartialEq)]
enum ArgError {
    NonASCII(char),
    NonAlphabetic(char),
    NonLowercase(char),
}

impl Display for ArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonASCII(c) => write!(f, "{c} is not an ASCII character"),
            Self::NonAlphabetic(c) => write!(f, "{c} is not an alphabetic character"),
            Self::NonLowercase(c) => write!(f, "{c} is not a lowercase alphabetical character"),
        }
    }
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
            return Err(ArgError::NonASCII(x));
        } else if !x.is_alphabetic() {
            return Err(ArgError::NonAlphabetic(x));
        } else if x.is_uppercase() {
            return Err(ArgError::NonLowercase(x));
        }
    }
    Ok(s.chars().map(|x: char| {x as u32 - 'a' as u32 + 1}).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_lowercase() {
        assert_eq!(lettersum("Hello"), Err(ArgError::NonLowercase('H')));
    }

    #[test]
    fn test_non_ascii() {
        assert_eq!(lettersum("ß"), Err(ArgError::NonASCII('ß')));
    }

    #[test]
    fn test_non_alphabetic() {
        assert_eq!(lettersum("123"), Err(ArgError::NonAlphabetic('1')));
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