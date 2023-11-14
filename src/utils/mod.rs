use std::env;
use std::string::String;
use aws_types::region::Region;

pub mod iam;
pub mod sts;

pub async fn load_config() -> aws_types::sdk_config::SdkConfig {
    let aws_region = env::var("AWS_REGION").unwrap();
    let region = Region::new(aws_region);
    aws_config::from_env().region(region).load().await
}
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