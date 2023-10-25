use std::error::Error;
use std::sync::Arc;
use aws_sdk_iam::operation::create_role::{CreateRoleError, CreateRoleOutput};
use aws_sdk_iam::types::{Policy, Role};
use aws_sdk_sts;
use aws_sdk_iam::error::SdkError;
use aws_sdk_iam::operation::create_policy::{CreatePolicyError, CreatePolicyOutput};
use aws_smithy_runtime_api::client::orchestrator::HttpResponse;
pub async fn check_role_exists(
	config: Arc<aws_config::SdkConfig>,
	role_name: &str,
) -> Result<(), Box<dyn Error>> {
	// Configure the AWS region and create an IAM client.
	let iam = aws_sdk_iam::Client::new(&config);

	// Create a request to get information about the role.
	let get_role_request = iam.get_role().role_name(role_name).send().await?;
	// Attempt to get information about the role.
	Ok(())
}

pub async fn create_policy(
	config: Arc<aws_config::SdkConfig>,
	policy_name: &str,
	policy_document: &str,
) -> Result<Policy, SdkError<CreatePolicyError,HttpResponse>> {
	// Configure the AWS region and create an IAM client.
	let iam = aws_sdk_iam::Client::new(&config);
	let r = iam.create_policy()
		.policy_name(policy_name)
		.policy_document(policy_document)
		.send()
		.await?;

	Ok(r.policy.unwrap())
}
pub async fn create_role(
	config: Arc<aws_config::SdkConfig>,
	role_name: &str,
	assume_role_policy_document: &str,
) -> Result<Role, SdkError<CreateRoleError>> {
	// Configure the AWS region and create an IAM client.
	let iam = aws_sdk_iam::Client::new(&config);

	let strippedPolicy = &assume_role_policy_document.to_string();
	let response = iam
		.create_role()
		.role_name(role_name)
		.assume_role_policy_document(strippedPolicy)
		.send()
		.await;
	match response {
		Ok(x) => Ok(x.role.unwrap()),
		Err(e) => {
			println!("create_role {:?}", e);
			return Err(e);
		}
	}
}
