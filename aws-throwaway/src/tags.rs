use aws_sdk_ec2::operation::describe_tags::DescribeTagsOutput;
use aws_sdk_ec2::types::Filter;
use aws_sdk_ec2::types::{ResourceType, Tag, TagSpecification};

/// Specify the cleanup process to use.
pub enum CleanupResources {
    /// Cleanup resources created by all [`Aws`] instances that use [`CleanupResources::WithAppTag`] of the same tag.
    /// It is highly reccomended that this tag is hardcoded, generating this tag could easily lead to forgotten resources.
    WithAppTag(String),
    /// Cleanup resources created by all [`Aws`] instances regardless of whether it was created via [`CleanupResources::AllResources`] or [`CleanupResources::ResourcesMatchingTag`]
    AllResources,
}

// include a magic number in the keyname to avoid collisions
// This can never change or we may fail to cleanup resources.
const USER_TAG_NAME: &str = "aws-throwaway-23c2d22c-d929-43fc-b2a4-c1c72f0b733f:user";
const APP_TAG_NAME: &str = "aws-throwaway-23c2d22c-d929-43fc-b2a4-c1c72f0b733f:app";

pub(crate) struct Tags {
    pub user_name: String,
    pub cleanup: CleanupResources,
}

impl Tags {
    pub fn create_tags(
        &self,
        resource_type: ResourceType,
        resource_name: &'static str,
    ) -> TagSpecification {
        let mut builder = TagSpecification::builder()
            .resource_type(resource_type)
            .tags(Tag::builder().key("Name").value(resource_name).build())
            .tags(
                Tag::builder()
                    .key(USER_TAG_NAME)
                    .value(&self.user_name)
                    .build(),
            );

        builder = match &self.cleanup {
            CleanupResources::WithAppTag(tag) => {
                builder.tags(Tag::builder().key(APP_TAG_NAME).value(tag).build())
            }
            CleanupResources::AllResources => builder,
        };

        builder.build()
    }

    pub async fn fetch_user_tags(
        &self,
        client: &aws_sdk_ec2::Client,
        resource_type: &str,
    ) -> DescribeTagsOutput {
        client
            .describe_tags()
            .set_filters(Some(vec![
                Filter::builder()
                    .name(format!("tag:{USER_TAG_NAME}"))
                    .values(&self.user_name)
                    .build(),
                Filter::builder()
                    .name("resource-type")
                    .values(resource_type)
                    .build(),
            ]))
            .send()
            .await
            .map_err(|e| e.into_service_error())
            .unwrap()
    }

    pub async fn fetch_app_tags(
        &self,
        client: &aws_sdk_ec2::Client,
        resource_type: &str,
    ) -> Option<DescribeTagsOutput> {
        match &self.cleanup {
            CleanupResources::WithAppTag(tag) => Some(
                client
                    .describe_tags()
                    .set_filters(Some(vec![
                        Filter::builder()
                            .name(format!("tag:{APP_TAG_NAME}"))
                            .values(tag)
                            .build(),
                        Filter::builder()
                            .name("resource-type")
                            .values(resource_type)
                            .build(),
                    ]))
                    .send()
                    .await
                    .map_err(|e| e.into_service_error())
                    .unwrap(),
            ),
            CleanupResources::AllResources => None,
        }
    }
}
