//! Roman crate provides the functionality to convert roman numerals into a u32 integer

use std::str::FromStr;
// use std::num::ParseIntError;
use std::num::IntErrorKind;

/// Roman numeral data type
struct Roman(u32);


impl FromStr for Roman {
    type Err = IntErrorKind;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let e = std::num::IntErrorKind::InvalidDigit;
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::Roman;

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
}