mod cpu_arch;
mod ec2_instance;
mod ec2_instance_definition;
mod iam;
mod ssh;
mod tags;

use aws_config::meta::region::RegionProviderChain;
use aws_config::SdkConfig;
use aws_sdk_ec2::config::Region;
use aws_sdk_ec2::types::{BlockDeviceMapping, EbsBlockDevice, KeyType, ResourceType, VolumeType};
use base64::Engine;
use ssh_key::rand_core::OsRng;
use ssh_key::PrivateKey;
use tags::Tags;
use uuid::Uuid;

pub use aws_sdk_ec2::types::InstanceType;
pub use ec2_instance::Ec2Instance;
pub use ec2_instance_definition::Ec2InstanceDefinition;
pub use tags::CleanupResources;

async fn config() -> SdkConfig {
    let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"));
    aws_config::from_env().region(region_provider).load().await
}

/// Construct this type to create and cleanup aws resources.
pub struct Aws {
    client: aws_sdk_ec2::Client,
    keyname: String,
    client_private_key: String,
    host_public_key: String,
    host_public_key_bytes: Vec<u8>,
    host_private_key: String,
    security_group: String,
    tags: Tags,
}

impl Aws {
    /// Construct a new [`Aws`]
    ///
    /// Before returning the [`Aws`], all preexisting resources conforming to the specified [`CleanupResources`] approach are destroyed.
    /// The specified [`CleanupResources`] is then also used by the [`Aws::cleanup_resources`] method.
    pub async fn new(cleanup: CleanupResources) -> Self {
        let config = config().await;
        let user_name = iam::user_name(&config).await;
        let keyname = format!("aws-throwaway-{user_name}-{}", Uuid::new_v4());
        let client = aws_sdk_ec2::Client::new(&config);

        let tags = Tags {
            user_name: user_name.clone(),
            cleanup,
        };

        // Cleanup any resources that were previously failed to cleanup
        Self::cleanup_resources_inner(&client, &tags).await;

        let keypair = client
            .create_key_pair()
            .key_name(&keyname)
            .key_type(KeyType::Ed25519)
            .tag_specifications(tags.create_tags(ResourceType::KeyPair, "aws-throwaway"))
            .send()
            .await
            .map_err(|e| e.into_service_error())
            .unwrap();
        let client_private_key = keypair.key_material().unwrap().to_string();
        tracing::info!("client_private_key:\n{}", client_private_key);

        let security_group = format!("aws-throwaway-{user_name}-{}", Uuid::new_v4());
        client
            .create_security_group()
            .group_name(&security_group)
            .description("aws-throwaway security group")
            .tag_specifications(tags.create_tags(ResourceType::SecurityGroup, "aws-throwaway"))
            .send()
            .await
            .map_err(|e| e.into_service_error())
            .unwrap();
        tracing::info!("created security group");
        assert!(client
            .authorize_security_group_ingress()
            .group_name(&security_group)
            .source_security_group_name(&security_group)
            .tag_specifications(
                tags.create_tags(ResourceType::SecurityGroupRule, "within aws-throwaway SG")
            )
            .send()
            .await
            .map_err(|e| e.into_service_error())
            .unwrap()
            .r#return()
            .unwrap());
        tracing::info!("created security group rule");
        assert!(client
            .authorize_security_group_ingress()
            .group_name(&security_group)
            .ip_protocol("tcp")
            .from_port(22)
            .to_port(22)
            .cidr_ip("0.0.0.0/0")
            .tag_specifications(tags.create_tags(ResourceType::SecurityGroupRule, "ssh"))
            .send()
            .await
            .map_err(|e| e.into_service_error())
            .unwrap()
            .r#return()
            .unwrap());
        tracing::info!("created security group rule");

        let key = PrivateKey::random(&mut OsRng {}, ssh_key::Algorithm::Ed25519).unwrap();
        let host_public_key_bytes = key.public_key().to_bytes().unwrap();
        let host_public_key = key.public_key().to_openssh().unwrap();
        let host_private_key = key.to_openssh(ssh_key::LineEnding::LF).unwrap().to_string();

        Aws {
            client,
            keyname,
            client_private_key,
            host_public_key_bytes,
            host_public_key,
            host_private_key,
            security_group,
            tags,
        }
    }

    /// Call before dropping [`Aws`]
    /// Uses the `CleanupResources` method specified in the constructor.
    pub async fn cleanup_resources(&self) {
        Self::cleanup_resources_inner(&self.client, &self.tags).await
    }

    /// Call to cleanup without constructing an [`Aws`]
    pub async fn cleanup_resources_static(cleanup: CleanupResources) {
        let config = config().await;
        let user_name = iam::user_name(&config).await;
        let client = aws_sdk_ec2::Client::new(&config);
        let tags = Tags { user_name, cleanup };
        Aws::cleanup_resources_inner(&client, &tags).await;
    }

    async fn get_all_throwaway_tags(
        client: &aws_sdk_ec2::Client,
        tags: &Tags,
        resource_type: &str,
    ) -> Vec<String> {
        let (user_tags, app_tags) = tokio::join!(
            tags.fetch_user_tags(client, resource_type),
            tags.fetch_app_tags(client, resource_type),
        );

        let mut ids_of_user = vec![];
        for tag in user_tags.tags().unwrap() {
            if let Some(id) = tag.resource_id() {
                ids_of_user.push(id.to_owned());
            }
        }

        if let Some(app_tags) = app_tags {
            let mut ids_of_user_and_app = vec![];
            for app_tag in app_tags.tags().unwrap() {
                if let Some(id) = app_tag.resource_id() {
                    let id = id.to_owned();
                    if ids_of_user.contains(&id) {
                        ids_of_user_and_app.push(id);
                    }
                }
            }
            ids_of_user_and_app
        } else {
            ids_of_user
        }
    }

    async fn cleanup_resources_inner(client: &aws_sdk_ec2::Client, tags: &Tags) {
        // delete instances
        tracing::info!("Terminating instances");
        let instance_ids = Self::get_all_throwaway_tags(client, tags, "instance").await;
        if !instance_ids.is_empty() {
            for result in client
                .terminate_instances()
                .set_instance_ids(Some(instance_ids))
                .send()
                .await
                .map_err(|e| e.into_service_error())
                .unwrap()
                .terminating_instances()
                .unwrap()
            {
                tracing::info!(
                    "Instance {:?} {:?} -> {:?}",
                    result.instance_id.as_ref().unwrap(),
                    result.previous_state().unwrap().name().unwrap(),
                    result.current_state().unwrap().name().unwrap()
                );
            }
        }

        // delete security groups
        for id in Self::get_all_throwaway_tags(client, tags, "security-group").await {
            if let Err(err) = client.delete_security_group().group_id(&id).send().await {
                tracing::info!(
                    "security group {id:?} could not be deleted, this will get cleaned up eventually on a future aws-throwaway cleanup: {:?}",
                    err.into_service_error().meta().message()
                )
            } else {
                tracing::info!("security group {id:?} was succesfully deleted",)
            }
        }

        // delete keypairs
        for id in Self::get_all_throwaway_tags(client, tags, "key-pair").await {
            client
                .delete_key_pair()
                .key_pair_id(&id)
                .send()
                .await
                .map_err(|e| {
                    anyhow::anyhow!(e.into_service_error())
                        .context(format!("Failed to delete keypair {id:?}"))
                })
                .unwrap();
            tracing::info!("keypair {id:?} was succesfully deleted");
        }
    }

    /// Creates a new EC2 instance as defined by [`Ec2InstanceDefinition`]
    pub async fn create_ec2_instance(&self, definition: Ec2InstanceDefinition) -> Ec2Instance {
        let result = self
            .client
            .run_instances()
            .instance_type(definition.instance_type.clone())
            .min_count(1)
            .max_count(1)
            .block_device_mappings(
                BlockDeviceMapping::builder().device_name("/dev/sda1").ebs(
                    EbsBlockDevice::builder()
                        .delete_on_termination(true)
                        .volume_size(definition.volume_size_gb as i32)
                        .volume_type(VolumeType::Gp2)
                        .build()
                ).build()
            )
            .security_groups(&self.security_group)
            .key_name(&self.keyname)
            .user_data(base64::engine::general_purpose::STANDARD.encode(format!(
                r#"#!/bin/bash
sudo systemctl stop ssh
echo "{}" > /etc/ssh/ssh_host_ed25519_key.pub
echo "{}" > /etc/ssh/ssh_host_ed25519_key

echo "ClientAliveInterval 30" >> /etc/ssh/sshd_config
sudo systemctl start ssh
            "#,
                self.host_public_key, self.host_private_key
            )))
            .tag_specifications(self.tags.create_tags(ResourceType::Instance, "aws-throwaway"))
            .image_id(format!(
                "resolve:ssm:/aws/service/canonical/ubuntu/server/22.04/stable/current/{}/hvm/ebs-gp2/ami-id",
                cpu_arch::get_arch_of_instance_type(definition.instance_type).get_ubuntu_arch_identifier()
            ))
            .send()
            .await
            .map_err(|e| e.into_service_error())
            .unwrap();
        let instance_id = result
            .instances()
            .unwrap()
            .iter()
            .next()
            .unwrap()
            .instance_id()
            .unwrap()
            .to_owned();

        let mut public_ip = None;
        let mut private_ip = None;

        while public_ip.is_none() || private_ip.is_none() {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            for reservation in self
                .client
                .describe_instances()
                .instance_ids(&instance_id)
                .send()
                .await
                .map_err(|e| e.into_service_error())
                .unwrap()
                .reservations()
                .unwrap()
            {
                for instance in reservation.instances().unwrap() {
                    public_ip = instance.public_ip_address().map(|x| x.parse().unwrap());
                    private_ip = instance.private_ip_address().map(|x| x.parse().unwrap());
                }
            }
        }
        let public_ip = public_ip.unwrap();
        let private_ip = private_ip.unwrap();
        tracing::info!("created EC2 instance at: {public_ip}");

        Ec2Instance::new(
            public_ip,
            private_ip,
            self.host_public_key_bytes.clone(),
            self.host_public_key.clone(),
            &self.client_private_key,
        )
        .await
    }
}
