//! This module takes a string and returns a specified number of words where the order
//! of words returned occur from most frequent to least frequent. Tie breakers are
//! ordered arbitrarily but most likely alphabetically according to language.

extern crate regex;
//extern crate ordermap;
use regex::Regex;
use std::collections::HashMap;
//use ordermap::OrderMap;


#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_string_vectorized() {
        let test_string = "(This) {is} <a> [test] okay! So, just relax...";
        let test_list = word_list(test_string);
        assert_eq!(vec!["this","is","a","test","okay","so","just","relax"], test_list);
    }

    #[test]
    fn test_string_vectorized_multiline() {
        let test_string = "(This) {is} <a> \n [test] okay! \n So, just relax...";
        let test_list = word_list(test_string);
        assert_eq!(vec!["this","is","a","test","okay","so","just","relax"], test_list);
    }

    #[test]
    fn test_mapping_and_ordering() {
        let test_string = "(This) {is} <a> [test] okay! this this, is";
        let test_map = word_freq_list(test_string, 3);
        assert_eq!(vec!["this","is","a"], test_map);
    }
}

/// Word List is a helper function made to vectorize an english string into a vector
fn word_list(text: &str) -> Vec<String> {
    let re = Regex::new(r"[0-9a-zA-Z\-]+").unwrap();
    let mut list: Vec<String> = vec![];

    for item in re.captures_iter(&text) {
        let word = item[0].to_string().to_lowercase();
        list.push(word);
    }

    list
}

/// Word Frequency List is a function that takes two arguments, a string and the
/// number of items to return. If the specified number of items to return is larger 
/// than the number of items that can be returned, the largest possible number will be
/// returned.
// TODO: combine word_list and word_freq_list later to speed up computation
pub fn word_freq_list(roman_text: &str, num_of_items: u64) -> Vec<&str> {
    assert!(num_of_items > 0);
    let list = word_list(roman_text);
    let mut map: HashMap<String, u64> = HashMap::new();

    vec!["returned", "value"]
}
