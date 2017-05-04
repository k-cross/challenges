extern crate wc;

static test_text: &'static str = "This this this will will be a very long long.";
static test_text2: &'static str = ". , .";
    

#[test]
fn return_specified_number_of_items_in_order() {
    let test_list = wc::word_freq_list(test_text, 7);
    assert_eq!(test_list[0], "this".to_string());
    assert!(test_list[1] == "long".to_string() || test_list[1] == "will".to_string());
    assert!(test_list[2] == "long".to_string() || test_list[2] == "will".to_string());
    assert_eq!(test_list.len(), 6);
}

#[test]
fn return_empty_vector_when_string_is_empty() {
    let test_list = wc::word_freq_list(test_text2, 10);
    assert_eq!(test_list.len(), 0);
}
