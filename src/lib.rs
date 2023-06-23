//! Roman crate provides the functionality to convert roman numerals into a ````u32``` integer
//! and from ```u32``` integers to roman numeral ```String```.
//! ## Example
//! ```rust
//!     use roman::Roman;
//! 
//!     let n = Roman::to_int("xxiv".to_string()).unwrap();
//!     assert_eq!(n, 24);
//! ```


/// Roman numeral data type
pub struct Roman;

impl Roman {
    /// Roman's ```to_int``` method is used to parse roman numerals into ```u32``` integers.
    /// 
    /// Parses a ```String``` v and returns a value of type ```Option<u32>```
    pub fn to_int(v: String) -> Option<u32> {
        convert_roman_to_number(&v)
    }

    /// Roman's ```from_int``` method is used to parse an unsigned integer into a ```String```.
    /// 
    /// Parses a ```u32``` input value and returns a value of type ```String```.
    fn from_int(v: u32) -> String {
        unimplemented!()
    }
    
}

/// ```get_number_from_roman``` function takes a ```char``` input and returns
/// 
/// an ```Option<u32>``` value
fn get_number_from_roman(ch: char) -> Option<u32> {
    match ch.to_ascii_uppercase() {
        'I' => Some(1),
        'V' => Some(5),
        'X' => Some(10),
        'L' => Some(50),
        'C' => Some(100),
        'D' => Some(500),
        'M' => Some(1000),
        _ => None
    }
}

/// ```convert_roman_to_number``` function parses a roman numeral into a number. It 
/// takes a ```&str``` ```v``` and returns a value of type ```Option<u32>```.
fn convert_roman_to_number<'a>(v: &'a str) -> Option<u32> {
    
    // value to hold the converted number string
    let mut converted = 0u32;
    let mut prev = 0u32;
    for ch in v.to_uppercase().chars() {
        // get the roman numeral number equivalent
        let number = get_number_from_roman(ch);
        if number.is_some() {
            if prev == 0 {
                converted = number.unwrap();
                prev = number.unwrap();
            } else {
                if prev == number.unwrap() || prev > number.unwrap() {
                    converted += number.unwrap();
                    prev = number.unwrap();
                } else {
                    if converted < number.unwrap() {
                        converted = number.unwrap() - prev;
                    } else {
                        converted -= prev;
                        converted += number.unwrap() - prev;
                    }
                    prev = number.unwrap();
                }
            }
        } else {
            return None;
        }
    }

    Some(converted)
}

#[cfg(test)]
mod tests {

    use crate::{get_number_from_roman, convert_roman_to_number};

    fn roman_main_numbers_inputs_output() -> Vec<(char, u32)> {
        vec![('I', 1), ('V', 5), ('X', 10), ('l', 50), ('c', 100), ('d', 500), ('m', 1000)]
    }

    #[test]
    fn test_get_number_from_rome() {
        for v in roman_main_numbers_inputs_output() {
            let n = get_number_from_roman(v.0).unwrap();
            assert_eq!(n, v.1);
        }
    }

    fn convert_roman_to_number_inputs_output<'a>() -> Vec<(&'a str, u32)> {
        vec![("I", 1), ("iV", 4), ("iX", 9), ("li", 51), ("cx", 110), ("mmxxiii", 2023), 
        ("dc", 600), ("ml", 1050), ("xxxvi", 36), ("xvI", 16), ("liv", 54),
        ("xlv", 45), ("XLII", 42), ("xiii", 13), ("miv", 1004), ("mmmiv", 3004)]
    }

    #[test]
    fn test_roman_to_number_converter() {
        for v in convert_roman_to_number_inputs_output() {
            let n = convert_roman_to_number(v.0).unwrap();
            assert_eq!(n, v.1);
        }
    }
}