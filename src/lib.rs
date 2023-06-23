//! Roman crate provides the functionality to convert roman numerals into a u32 integer

use std::collections::HashMap;

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

/// ```get_number_from_roman``` function takes a ```&str``` input and returns
/// 
/// an ```Option<u32>``` value
fn get_number_from_roman<'a>(v: &'a str) -> Option<u32> {
    let mut rome: HashMap<String, u32> = HashMap::new();
    rome.insert("I".to_string(), 1);
    rome.insert("V".to_string(), 5);
    rome.insert("X".to_string(), 10);
    rome.insert("L".to_string(), 50);
    rome.insert("C".to_string(), 100);
    rome.insert("D".to_string(), 500);
    rome.insert("M".to_string(), 1000);

    let s = v.to_string().to_uppercase();

    if let Some(x) = rome.get(&s) {
        Some(x.to_owned())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use crate::{get_number_from_roman};

    fn roman_main_numbers<'a>() -> Vec<(&'a str, u32)> {
        vec![("I", 1), ("V", 5), ("X", 10), ("l", 50), ("c", 100), ("d", 500), ("m", 1000)]
    }

    #[test]
    fn test_get_number_from_rome() {
        for v in roman_main_numbers() {
            let n = get_number_from_roman(v.0).unwrap();
            assert_eq!(n, v.1);
        }
    }
}