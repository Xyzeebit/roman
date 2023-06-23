//! Roman crate provides the functionality to convert roman numerals into a u32 integer

/// Roman numeral data type

pub struct Roman;

impl Roman {
    /// Roman's ```to_int``` method is used to parse roman numerals into ```u32``` integers.
    /// 
    /// Parses a string v and returns a value of type ```u32```
    fn to_int(v: String) -> u32 {
        unimplemented!()
    }

    /// Roman's ```from_int``` method is used to parse an unsigned integer into a ```String```.
    /// 
    /// Parses a ```u32``` input value and returns a value of type ```String```.
    fn from_int(v: u32) -> String {
        unimplemented!()
    }
    
}

#[cfg(test)]
mod tests {

    use crate::{Roman};

    fn roman_input_and_output<'a>() -> Vec<(&'a str, u32)> {
        vec![("i", 1), ("ii", 2), ("iii", 3), ("iv", 4), ("v", 5)]
    }

    #[test]
    fn test_roman() {
        for v in roman_input_and_output() {
            // let n = Rome::new(v.0);
            // assert_eq!(v.1, n.to_int().unwrap());
        }
    }
}