//! Roman crate provides the functionality to convert roman numerals into a u32 integer

use std::str::FromStr;
use std::num::ParseIntError;

/// Roman numeral data type
struct Roman(String);

impl Roman {
    /// Creates a Roman instance
    fn new(value: &str) -> Self {
        Roman(value.to_string())
    }
}

impl FromStr for Roman {
    
}

#[cfg(test)]
mod tests {
    use crate::Roman;

    fn roman_input_and_output<'a>() -> Vec<(&'a str, u32)> {
        vec![("i", 1), ("ii", 2), ("iii", 3), ("iv", 4), ("v", 5)]
    }

    #[test]
    fn test_parse() {
        for v in roman_input_and_output() {
            let rm = Roman::new(v.0);
            let n = rm.parse().unwrap();
            assert_eq!(v.1, n);
        }
    }
}