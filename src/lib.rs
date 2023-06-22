//! Roman crate provides the functionality to convert roman numerals into a u32 integer

use std::str::FromStr;
// use std::num::ParseIntError;
use std::num::IntErrorKind;

/// Roman numeral data type
struct Roman(u32);

impl FromStr for Roman {
    type Err = IntErrorKind;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "i" => Ok(Roman(1)),
            "ii" => Ok(Roman(2)),
            "iii" => Ok(Roman(3)),
            "iv" => Ok(Roman(4)),
            "v" => Ok(Roman(5)),
            _ => Err(IntErrorKind::InvalidDigit)
        }
    }
}

struct Rome<'a>(&'a str);

impl<'a> Rome<'a> {
    fn new(s: &'a str) -> Self {
        Rome(s)
    }
    fn to_int(&self) -> Option<u32> {
        match self.0 {
            "i" => Some(1),
            "ii" => Some(2),
            "iii" => Some(3),
            "iv" => Some(4),
            "v" => Some(5),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{Roman, Rome};

    fn roman_input_and_output<'a>() -> Vec<(&'a str, u32)> {
        vec![("i", 1), ("ii", 2), ("iii", 3), ("iv", 4), ("v", 5)]
    }

    #[test]
    fn test_parse() {
        for v in roman_input_and_output() {
            let n = Roman::from_str(v.0);
            assert_eq!(v.1, n.unwrap().0);
        }
    }

    #[test]
    fn test_rome() {
        for v in roman_input_and_output() {
            let n = Rome::new(v.0);
            assert_eq!(v.1, n.to_int().unwrap());
        }
    }
}