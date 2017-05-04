//! # License
//! 
//! Public Domain :)
//! 
//! # Extra
//! 
//! This module takes a string and returns a specified number of words where the order
//! of words returned occur from most frequent to least frequent. Tie breakers are
//! ordered arbitrarily but most likely alphabetically according to language. Another
//! interesting thing about the function is that it returns the length of unique words
//! in order if a higher number was specified.
//! 
//! Since N is the number of characters and not the number of words, I use an average
//! word length of 5.2 characters. This number was just taken to represent the average
//! word length in documents, not in the dictionary, and was taken from a quora post,
//! not calculated. Since O(N/5.2) is equal to O(N) I will just use words as meaning
//! O(N) in further calculations.
//! 
//! First the string is run through a regex which breaks apart words and stores them 
//! into a map. This happens N times giving us a runtime of O(N). Next, we have K items
//! that need to be returned in an order from most frequent to least frequent. If we 
//! let K = N then N sorts are needed which gives us a worst case runtime of 
//! O(N log N) thus making the req. unsatiatiable for worst case. But since this is 
//! essentially bucket sort, we achieve an average runtime of O(N).
//! 
//! The space complexity is also comparable, especially since rust uses a lot of 
//! in-place methods to change data structures which means no real extra space is used,
//! only the space needed to store words and ultimately the only used space is the 
//! words returned in the vector.
//! 
//! # Examples
//! ```
//! let test_string = "<This> is {a} [test]... This should be, relaxing?";
//! let word_freq = wc::word_freq_list(test_string, 3);
//! 
//! assert_eq!(word_freq.len(), 3);
//! 
//! for word in word_freq {
//!     println!("{}", &word);
//! }
//! ```

extern crate regex;
use regex::Regex;
use std::collections::HashMap;


#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_string_vectorized_multiline() {
        let test_string = "(This) {is} <a> \n [test] okay! \n So, I'll just relax...";
        let test_list = word_map(test_string);
        assert!(test_list.contains_key("this"));
        assert!(test_list.contains_key("is"));
        assert!(test_list.contains_key("a"));
        assert!(test_list.contains_key("test"));
        assert!(test_list.contains_key("okay"));
        assert!(test_list.contains_key("so"));
        assert!(test_list.contains_key("i'll"));
        assert!(test_list.contains_key("just"));
        assert!(test_list.contains_key("relax"));
        assert!(!test_list.contains_key("..."));
        assert!(!test_list.contains_key(" "));
        assert!(!test_list.contains_key(""));
    }

    #[test]
    fn test_standard_mapping_and_ordering() {
        let test_string = "(This) {is} <a> [test] okay! this this, is";
        let test_map = word_freq_list(test_string, 3);
        assert_eq!("this".to_string(), test_map[0]);
        assert_eq!("is".to_string(), test_map[1]);
        assert_eq!(test_map.len(), 3);
    }

    #[test]
    fn test_out_of_range_mapping_and_ordering() {
        let test_string = "(This) {is} <a> [test] okay! this this, is";
        let test_map = word_freq_list(test_string, 20);
        assert_eq!("this".to_string(), test_map[0]);
        assert_eq!("is".to_string(), test_map[1]);
        assert_eq!(test_map.len(), 5);
    }
}

/// Word List is a helper function made to vectorize an english string into a vector
fn word_map(text: &str) -> HashMap<String, u64> {
    let re = Regex::new(r"[0-9a-zA-Z\-']+").unwrap();
    //let mut list: Vec<String> = Vec::new();
    let mut map: HashMap<String, u64> = HashMap::new();

    for item in re.captures_iter(&text) {
        let word = item[0].to_string().to_lowercase();
        *map.entry(word).or_insert(0) += 1;
    }

    map
}

/// Word Frequency List is a function that takes two arguments, a string and the
/// number of items to return. If the specified number of items to return is larger 
/// than the number of items that can be returned, the largest possible number will be
/// returned.
/// 
/// # Examples
/// 
/// ```
/// use wc::word_freq_list;
/// let test_string = "<This> is {a} [test]... This should be, relaxing?";
/// let word_freq = word_freq_list(test_string, 3);
/// 
/// assert_eq!(word_freq.len(), 3);
/// ```
pub fn word_freq_list(roman_text: &str, num_of_items: usize) -> Vec<String> {
    assert!(num_of_items > 0);

    let map = word_map(roman_text);

    let mut mapped_words: Vec<(String, u64)> = Vec::new();
    for (k,v) in map {
        mapped_words.push((k,v));
    }
    mapped_words.sort_by(|a,b| a.1.cmp(&b.1));

    let mut freqent_words: Vec<(String)> = Vec::new();

    if num_of_items > mapped_words.len() {
        for i in 0..mapped_words.len() {
            freqent_words.push(mapped_words.pop().unwrap().0);
        }
    }
    else {
        for i in 0..num_of_items {
            freqent_words.push(mapped_words.pop().unwrap().0);
        }
    }

    freqent_words
}
