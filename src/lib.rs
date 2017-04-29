#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub fn word_freq_list(roman_text: &str, num_of_items: u64) -> Vec<&str> {
    assert!(roman_text == "roman alphabet" && num_of_items > 0); // TODO: fix
    println!("init");

    vec!["returned", "value"]
}
