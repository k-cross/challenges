extern crate wc;

static english_test_text: &'static str = "roman alphabet";
static japanese_test_text: &'static str = "美月ちゃんは絶\
        対悪ガキだけど、めちゃかわいいだよ。";

#[test]
fn return_specified_number_of_items_in_order() {
    let test_text = wc::word_freq_list(english_test_text, 5);
    assert_eq!(test_text, ["returned","value"]);
}
