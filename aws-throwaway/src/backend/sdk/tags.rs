use crate::CleanupResources;
use aws_sdk_ec2::operation::describe_tags::DescribeTagsOutput;
use aws_sdk_ec2::types::Filter;
use aws_sdk_ec2::types::{ResourceType, Tag, TagSpecification};

pub(crate) struct Tags {
    pub user_name: String,
    pub cleanup: CleanupResources,
}

impl Tags {
    pub fn create_tags(
        &self,
        resource_type: ResourceType,
        resource_name: &str,
    ) -> TagSpecification {
        let mut builder = TagSpecification::builder()
            .resource_type(resource_type)
            .tags(Tag::builder().key("Name").value(resource_name).build())
            .tags(
                Tag::builder()
                    .key(crate::USER_TAG_NAME)
                    .value(&self.user_name)
                    .build(),
            );

        builder = match &self.cleanup {
            CleanupResources::WithAppTag(tag) => {
                builder.tags(Tag::builder().key(crate::APP_TAG_NAME).value(tag).build())
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
                    .name(format!("tag:{}", crate::USER_TAG_NAME))
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
                            .name(format!("tag:{}", crate::APP_TAG_NAME))
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
