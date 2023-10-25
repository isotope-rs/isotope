use aws_sdk_sts::error::SdkError;
use aws_sdk_sts::operation::assume_role::{AssumeRoleError, AssumeRoleOutput};
use aws_smithy_runtime_api::client::orchestrator::HttpResponse;
use std::error::Error;
use std::sync::Arc;

pub async fn get_account_id(config: Arc<aws_config::SdkConfig>) -> Option<String> {
    let sts = aws_sdk_sts::Client::new(&config);
    let response = sts.get_caller_identity().send().await;
    let account_id = response.unwrap().account.unwrap();
    Some(account_id)
}

pub async fn assume_role(
    config: Arc<aws_config::SdkConfig>,
    account_id: &str,
    role_name: &str,
    session_name: &str,
)  {
    let sts = aws_sdk_sts::Client::new(&config);
    let input_role = format!("arn:aws:iam::{}:role/{}", account_id, role_name);
    println!("Assuming {}", input_role);
    let sts_response = sts
        .assume_role()
        .role_arn(input_role)
        // e.g. DetectPublicS3Buckets
        .role_session_name(session_name)
        .send()
        .await;

    println!("{:?}", sts_response);
   // Ok(sts_response)
}
