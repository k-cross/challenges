extern crate regex;
use regex::Regex;


/// This function is made to vectorize an english string into a list
// TODO: Change approach, needs to run in O(n) and this might be too expensive.
pub fn word_list(text: &str) -> Vec<&str> {
    let re = Regex::new(r"").unwrap();
    let mut list = vec![];

    list
}

pub fn word_freq_list(roman_text: &str, num_of_items: u64) -> Vec<&str> {
    assert!(roman_text == "roman alphabet" && num_of_items > 0); // TODO: fix
    println!("init");

    vec!["returned", "value"]
}


#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_string_vectorized() {
        let test_string = "(This) {is} <a> [test] okay! So, just relax...";
        let test_list = super::word_list(test_string);
        assert_eq!(vec!["This","is","a","test","okay","So","just","relax"], test_list);
    }
}
