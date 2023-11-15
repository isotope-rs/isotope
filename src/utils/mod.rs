use std::env;
use std::string::String;
use aws_config::default_provider::region::DefaultRegionChain;
use aws_types::region::Region;

pub mod iam;
pub mod sts;

/// Retrieves the AWS region from the "AWS_REGION" environment variable if set,
/// otherwise falls back to the default region determined by the DefaultRegionChain.
async fn get_region() -> Region {
    env::var("AWS_REGION")
        .map(Region::new)
        .unwrap_or({
            DefaultRegionChain::builder()
                .build()
                .region()
                .await
                .expect("Failed to determine the AWS region.")
        })
}

pub async fn load_config() -> aws_types::sdk_config::SdkConfig {
    let region = get_region().await;
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