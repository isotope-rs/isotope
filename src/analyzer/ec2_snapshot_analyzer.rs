use crate::analyzer::analyzer_trait;
use crate::analyzer::analyzer_trait::Analyzer;
use crate::analyzer::types::AnalysisResults;
use async_trait::async_trait;
use aws_sdk_ec2;
use aws_sdk_ec2::types::{Filter, PermissionGroup, SnapshotAttributeName};
use aws_types::sdk_config::SdkConfig;

pub struct EC2SnapshotAnalyzer {
    pub config: SdkConfig,
}

#[async_trait]
impl analyzer_trait::Analyzer for EC2SnapshotAnalyzer {
    async fn run(&self) -> Option<Vec<AnalysisResults>> {
        let mut results = Vec::new();
        let ec2 = aws_sdk_ec2::Client::new(&self.config);

        // Filter to include only your snapshots if needed
        let filter = Filter::builder()
            .set_name(Some("owner-id".to_string()))
            .build();

        if let Ok(snapshots) = ec2
            .describe_snapshots()
            .set_filters(Some(vec![filter]))
            .send()
            .await
        {
            for snapshot in snapshots.snapshots.unwrap_or_default() {
                let snapshot_id = snapshot.snapshot_id.clone().unwrap();

                // Check if the snapshot is public
                let attributes = ec2
                    .describe_snapshot_attribute()
                    .attribute(SnapshotAttributeName::CreateVolumePermission)
                    .snapshot_id(&snapshot_id)
                    .send()
                    .await;

                if let Ok(attrs) = attributes {
                    for perm in attrs.create_volume_permissions.unwrap_or_default() {
                        if perm.group == Some(PermissionGroup::All) {
                            results.push(AnalysisResults {
								message: format!("Public EC2 Snapshot: {}", snapshot_id),
								advice: "Consider making this snapshot private if it contains sensitive data.".to_string(),
								analyzer_name: self.get_name(),
							});
                        }
                    }
                }
            }
        }

        Some(results)
    }

    fn get_name(&self) -> String {
        "ec2_snapshot".to_string()
    }
}
