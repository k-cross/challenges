//! This program assumes that the language is English, but it can be easily extended to
//! include other Roman Alphabet based languages.

extern crate wc;

// testing
extern crate ordermap;
use ordermap::OrderMap;
use std::collections::HashMap;

fn main() {
    println!("init");
    let rmnabt: &str = "roman alphabet";
    let test_string = "(This) {is} <a> [test] okay! So, just relax...";
    let freq_list = wc::word_freq_list(&rmnabt, 2);
    let mut om = OrderMap::new();
    let mut hm = HashMap::new();

    hm.insert("a", 1);
    hm.insert("a", 1);
    hm.insert("b", 1);
    hm.insert("c", 1);
    hm.insert("d", 3);
    hm.insert("e", 1);

 //   let mut mapVec: Vec<_> = vec![];

    om.insert("a", 1);
    om.insert("b", 1);
    om.insert("c", 1);
    om.insert("d", 3);
    om.insert("e", 1);

    for (k,v) in &hm {
        println!("{} {}",k,v);
    }

    println!("{}", &freq_list[0]);
}
