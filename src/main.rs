extern crate wc;

fn main() {
    println!("init");
    let rmnabt: &str = "roman alphabet";
    let freq_list = wc::word_freq_list(&rmnabt, 2);
    println!("{}", &freq_list[0]);
}
