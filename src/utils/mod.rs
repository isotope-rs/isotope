use std::string::String;
pub mod iam;
pub mod sts;

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

#[test]
fn remove_whitespace_test() {

    let mut input = String::from("input output");
    let alen = input.len();
    remove_whitespace( &mut input);
    let blen = input.len();
    assert_ne!(alen,blen);
}