mod instance_type;
mod placement_strategy;

use crate::{
    backend::cli::instance_type::get_arch_of_instance_type, AwsBuilder, CleanupResources,
    Ec2Instance, Ec2InstanceDefinition, InstanceOs, NetworkInterface, APP_TAG_NAME, USER_TAG_NAME,
};
use anyhow::{anyhow, Result};
use base64::Engine;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
pub use instance_type::InstanceType;
pub use placement_strategy::PlacementStrategy;
use serde::Deserialize;
use ssh_key::{rand_core::OsRng, PrivateKey};
use std::future::Future;
use std::pin::Pin;
use std::{
    net::IpAddr,
    time::{Duration, Instant},
};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct Ignore {}

#[derive(Debug, Deserialize)]
struct Tag {
    #[serde(alias = "ResourceId")]
    resource_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Subnet {
    #[serde(alias = "SubnetId")]
    subnet_id: Option<String>,
    #[serde(alias = "MapPublicIpOnLaunch")]
    map_public_ip_on_launch: Option<bool>,
}

const AZ: &str = "us-east-1c";

struct Tags {
    pub user_name: String,
    pub cleanup: CleanupResources,
}
impl Tags {
    pub fn create_tags(&self, resource_type: &str, resource_name: &str) -> String {
        let user_name = &self.user_name;
        let app_tag = match &self.cleanup {
            CleanupResources::WithAppTag(tag) => {
                format!(",{{Key={APP_TAG_NAME},Value={tag}}}")
            }
            CleanupResources::AllResources => String::new(),
        };

        format!(
            "ResourceType={resource_type},Tags=[{{Key=Name,Value={resource_name}}},{{Key={USER_TAG_NAME},Value={user_name}}}{app_tag}]"
        )
    }

    pub async fn fetch_user_tags(&self, resource_type: &str) -> Vec<Tag> {
        #[derive(Debug, Deserialize)]
        enum TagResponse {
            Tags(Vec<Tag>),
        }
        let TagResponse::Tags(tags) = run_command(&[
            "ec2",
            "describe-tags",
            "--filters",
            &format!("Name=tag:{USER_TAG_NAME},Values={}", &self.user_name),
            &format!("Name=resource-type,Values={resource_type}"),
        ])
        .await
        .unwrap();

        tags
    }

    pub async fn fetch_app_tags(&self, resource_type: &str) -> Option<Vec<Tag>> {
        #[derive(Debug, Deserialize)]
        enum TagResponse {
            Tags(Vec<Tag>),
        }
        match &self.cleanup {
            CleanupResources::WithAppTag(tag) => {
                let TagResponse::Tags(tags) = run_command(&[
                    "ec2",
                    "describe-tags",
                    "--filters",
                    &format!("Name=tag:{APP_TAG_NAME},Values={tag}"),
                    &format!("Name=resource-type,Values={resource_type}"),
                ])
                .await
                .unwrap();
                Some(tags)
            }
            CleanupResources::AllResources => None,
        }
    }
}

pub struct Aws {
    tags: Tags,
    keyname: String,
    client_private_key: String,
    host_public_key: String,
    host_public_key_bytes: Vec<u8>,
    host_private_key: String,
    security_group_id: String,
    placement_group_name: String,
    subnet_id: String,
    subnet_map_public_ip_on_launch: bool,
    use_public_addresses: bool,
}

impl Aws {
    pub fn builder(cleanup: CleanupResources) -> AwsBuilder {
        AwsBuilder::new(cleanup)
    }

    pub(crate) async fn new(builder: AwsBuilder) -> Self {
        std::env::set_var("AWS_DEFAULT_REGION", "us-east-1");
        let user_name = user_name().await;
        let keyname = format!("aws-throwaway-{user_name}-{}", Uuid::new_v4());
        let security_group_name = format!("aws-throwaway-{user_name}-{}", Uuid::new_v4());
        let placement_group_name = format!("aws-throwaway-{user_name}-{}", Uuid::new_v4());

        let tags = Tags {
            user_name,
            cleanup: builder.cleanup,
        };

        Self::cleanup_resources_inner(&tags).await;

        let (client_private_key, security_group_id, _, subnet) = tokio::join!(
            Aws::create_key_pair(&tags, &keyname),
            Aws::create_security_group(
                &tags,
                &security_group_name,
                &builder.vpc_id,
                builder.security_group_id,
                &builder.expose_ports_to_internet
            ),
            Aws::create_placement_group(&tags, &placement_group_name, builder.placement_strategy),
            Aws::get_subnet(builder.subnet_id)
        );

        let subnet_id = subnet.subnet_id.unwrap();
        let subnet_map_public_ip_on_launch = subnet.map_public_ip_on_launch.unwrap();

        let key = PrivateKey::random(&mut OsRng {}, ssh_key::Algorithm::Ed25519).unwrap();
        let host_public_key_bytes = key.public_key().to_bytes().unwrap();
        let host_public_key = key.public_key().to_openssh().unwrap();
        let host_private_key = key.to_openssh(ssh_key::LineEnding::LF).unwrap().to_string();

        let use_public_addresses = builder.use_public_addresses;

        Aws {
            use_public_addresses,
            keyname,
            client_private_key,
            host_public_key_bytes,
            host_public_key,
            host_private_key,
            security_group_id,
            placement_group_name,
            subnet_id,
            subnet_map_public_ip_on_launch,
            tags,
        }
    }

    async fn create_key_pair(tags: &Tags, name: &str) -> String {
        #[derive(Debug, Deserialize)]
        struct KeyPair {
            #[serde(alias = "KeyMaterial")]
            material: String,
        }
        let result: KeyPair = run_command(&[
            "ec2",
            "create-key-pair",
            "--key-type",
            "ed25519",
            "--key-name",
            &name,
            "--tag-specifications",
            &tags.create_tags("key-pair", "aws-throwaway"),
        ])
        .await
        .unwrap();
        tracing::info!("client_private_key:\n{}", result.material);
        result.material
    }

    async fn create_security_group(
        tags: &Tags,
        name: &str,
        vpc_id: &Option<String>,
        security_group_id: Option<String>,
        ports: &[u16],
    ) -> String {
        match security_group_id {
            Some(id) => id,
            None => {
                #[derive(Debug, Deserialize)]
                struct SecurityGroup {
                    #[serde(alias = "GroupId")]
                    group_id: String,
                }
                let tag_specifications = tags.create_tags("security-group", "aws-throwaway");
                let mut command = vec![
                    "ec2",
                    "create-security-group",
                    "--group-name",
                    &name,
                    "--description",
                    "aws-throwaway security group",
                    "--tag-specifications",
                    &tag_specifications,
                ];
                if let Some(vpc_id) = vpc_id.as_ref() {
                    command.push("--vpc-id");
                    command.push(vpc_id);
                }
                let result: SecurityGroup = run_command(&command).await.unwrap();
                tracing::info!("created security group");

                let mut futures = FuturesUnordered::<Pin<Box<dyn Future<Output = ()>>>>::new();
                futures.push(Box::pin(Aws::create_ingress_rule_internal(tags, name)));
                if !ports.contains(&22) {
                    // SSH
                    futures.push(Box::pin(Aws::create_ingress_rule_for_port(tags, name, 22)));
                }
                for port in ports {
                    futures.push(Box::pin(Aws::create_ingress_rule_for_port(
                        tags, name, *port,
                    )));
                }
                while futures.next().await.is_some() {}

                result.group_id
            }
        }
    }

    async fn create_ingress_rule_internal(tags: &Tags, group_name: &str) {
        let _result: Ignore = run_command(&[
            "ec2",
            "authorize-security-group-ingress",
            "--group-name",
            &group_name,
            "--source-security-group-name",
            &group_name,
            "--tag-specifications",
            &tags.create_tags("security-group-rule", "within aws-throwaway SG"),
        ])
        .await
        .unwrap();
        tracing::info!("created security group rule - internal");
    }

    async fn create_ingress_rule_for_port(tags: &Tags, group_name: &str, port: u16) {
        let port = port.to_string();
        let _result: Ignore = run_command(&[
            "ec2",
            "authorize-security-group-ingress",
            "--group-name",
            &group_name,
            "--ip-protocol",
            "tcp",
            "--from-port",
            &port,
            "--to-port",
            &port,
            "--cidr-ip",
            "0.0.0.0/0",
            "--tag-specifications",
            &tags.create_tags("security-group-rule", &format!("port {port}")),
        ])
        .await
        .unwrap();
        tracing::info!("created security group rule - port {port}");
    }

    async fn create_placement_group(tags: &Tags, name: &str, strategy: PlacementStrategy) {
        let _result: Ignore = run_command(&[
            "ec2",
            "create-placement-group",
            "--group-name",
            &name,
            "--strategy",
            strategy.as_str(),
            "--tag-specifications",
            &tags.create_tags("placement-group", "aws-throwaway"),
        ])
        .await
        .unwrap();
        tracing::info!("created placement group");
    }

    async fn get_subnet(subnet_id: Option<String>) -> Subnet {
        let mut command = vec!["ec2", "describe-subnets", "--filters"];

        // this is needed to keep the string values alive for the length of the borrow
        let value;

        match subnet_id {
            Some(subnet_id) => command.push({
                value = format!("Name=subnet-id,Values={subnet_id}");
                &value
            }),
            None => {
                command.push("Name=default-for-az,Values=true");
                command.push({
                    value = format!("Name=availability-zone,Values={AZ}");
                    &value
                });
            }
        }

        #[derive(Debug, Deserialize)]
        enum Response {
            Subnets(Vec<Subnet>),
        }
        let Response::Subnets(mut result) = run_command(&command).await.unwrap();

        result.pop().unwrap()
    }

    /// Call before dropping [`Aws`]
    /// Uses the `CleanupResources` method specified in the constructor.
    pub async fn cleanup_resources(&self) {
        Self::cleanup_resources_inner(&self.tags).await
    }

    /// Call to cleanup without constructing an [`Aws`]
    pub async fn cleanup_resources_static(cleanup: CleanupResources) {
        std::env::set_var("AWS_DEFAULT_REGION", "us-east-1");
        let user_name = user_name().await;
        let tags = Tags { user_name, cleanup };
        Self::cleanup_resources_inner(&tags).await
    }

    async fn get_all_throwaway_tags(tags: &Tags, resource_type: &str) -> Vec<String> {
        let (user_tags, app_tags) = tokio::join!(
            tags.fetch_user_tags(resource_type),
            tags.fetch_app_tags(resource_type),
        );

        let mut ids_of_user = vec![];
        for tag in user_tags {
            if let Some(id) = tag.resource_id {
                ids_of_user.push(id.to_owned());
            }
        }

        if let Some(app_tags) = app_tags {
            let mut ids_of_user_and_app = vec![];
            for app_tag in app_tags {
                if let Some(id) = app_tag.resource_id {
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

    async fn cleanup_resources_inner(tags: &Tags) {
        // release elastic ips
        for id in Self::get_all_throwaway_tags(tags, "elastic-ip").await {
            run_command_empty_response(&["ec2", "release-address", "--allocation-id", &id])
                .await
                .map_err(|e| e.context(format!("Failed to release elastic ip {id:?}")))
                .unwrap();
            tracing::info!("elastic ip {id:?} was succesfully deleted");
        }

        tracing::info!("Terminating instances");
        let instance_ids = Self::get_all_throwaway_tags(tags, "instance").await;
        if !instance_ids.is_empty() {
            #[derive(Debug, Deserialize)]
            enum Response {
                TerminatingInstances(Vec<TerminatingInstance>),
            }
            #[derive(Debug, Deserialize)]
            struct TerminatingInstance {
                #[serde(alias = "CurrentState")]
                current_state: State,
                #[serde(alias = "PreviousState")]
                previous_state: State,
                #[serde(alias = "InstanceId")]
                instance_id: String,
            }
            #[derive(Debug, Deserialize)]
            struct State {
                #[serde(alias = "Name")]
                name: String,
            }

            let mut command = vec!["ec2", "terminate-instances", "--instance-ids"];
            command.extend(instance_ids.iter().map(|x| x.as_str()));

            let Response::TerminatingInstances(results) = run_command(&command).await.unwrap();
            for result in results {
                tracing::info!(
                    "Instance {:?} {:?} -> {:?}",
                    result.instance_id,
                    result.previous_state.name,
                    result.current_state.name,
                );
            }
        }

        tokio::join!(
            Aws::delete_security_groups(tags),
            Aws::delete_placement_groups(tags),
            Aws::delete_keypairs(tags),
        );
    }

    async fn delete_security_groups(tags: &Tags) {
        for id in Self::get_all_throwaway_tags(tags, "security-group").await {
            let result =
                run_command_empty_response(&["ec2", "delete-security-group", "--group-id", &id])
                    .await;
            if let Err(err) = result {
                tracing::info!(
                    "security group {id:?} could not be deleted, this will get cleaned up eventually on a future aws-throwaway cleanup: {:?}",
                    err
                )
            } else {
                tracing::info!("security group {id:?} was succesfully deleted")
            }
        }
    }

    async fn delete_placement_groups(tags: &Tags) {
        let placement_group_ids = Self::get_all_throwaway_tags(tags, "placement-group").await;
        if !placement_group_ids.is_empty() {
            // placement groups can not be deleted by id so we must look up their names
            #[derive(Debug, Deserialize)]
            enum Response {
                PlacementGroups(Vec<PlacementGroup>),
            }
            #[derive(Debug, Deserialize)]
            struct PlacementGroup {
                #[serde(alias = "GroupName")]
                group_name: String,
            }
            let mut command = vec!["ec2", "describe-placement-groups", "--group-ids"];
            for id in &placement_group_ids {
                command.push(id);
            }
            let Response::PlacementGroups(placement_groups) = run_command(&command).await.unwrap();
            for placement_group in placement_groups {
                let name = &placement_group.group_name;
                let result = run_command_empty_response(&[
                    "ec2",
                    "delete-placement-group",
                    "--group-name",
                    name,
                ])
                .await;
                if let Err(err) = result {
                    tracing::info!(
                        "placement group {name:?} could not be deleted, this will get cleaned up eventually on a future aws-throwaway cleanup: {err:?}",
                    )
                } else {
                    tracing::info!("placement group {name:?} was succesfully deleted")
                }
            }
        }
    }

    async fn delete_keypairs(tags: &Tags) {
        for id in Self::get_all_throwaway_tags(tags, "key-pair").await {
            let result: Result<Ignore> =
                run_command(&["ec2", "delete-key-pair", "--key-pair-id", &id]).await;
            if let Err(err) = result {
                if err.to_string().contains("UnauthorizedOperation") {
                    tracing::error!("{:?}", anyhow!(err).context(format!(
                        "Did not have permissions to delete keypair {id:?}, skipping all other keypairs since they will also fail."
                    )));
                    return;
                } else {
                    panic!("Failed to delete keypair {id:?}: {err:?}")
                }
            } else {
                tracing::info!("keypair {id:?} was succesfully deleted");
            }
        }
    }

    /// Creates a new EC2 instance as defined by [`Ec2InstanceDefinition`]
    pub async fn create_ec2_instance(&self, definition: Ec2InstanceDefinition) -> Ec2Instance {
        // elastic IP's are a limited resource so only create it if we truly need it.
        let elastic_ip = if self.use_public_addresses && definition.network_interface_count > 1 {
            #[derive(Debug, Deserialize)]
            struct ElasticIp {
                #[serde(alias = "PublicIp")]
                public_ip: String,
                allocation_id: String,
            }
            let result: ElasticIp = run_command(&[
                "ec2",
                "allocate-address",
                "--tag-specifications",
                &self.tags.create_tags("elastic-Ip", "aws-throwaway"),
            ])
            .await
            .unwrap();
            Some(result)
        } else {
            None
        };

        let ubuntu_version = match definition.os {
            InstanceOs::Ubuntu20_04 => "20.04",
            InstanceOs::Ubuntu22_04 => "22.04",
        };
        let image_id = definition.ami.unwrap_or_else(|| format!(
            "resolve:ssm:/aws/service/canonical/ubuntu/server/{}/stable/current/{}/hvm/ebs-gp2/ami-id",
            ubuntu_version,
            get_arch_of_instance_type(definition.instance_type.clone()).get_ubuntu_arch_identifier()
        ));
        #[derive(Debug, Deserialize)]
        struct RunInstancesResponse {
            #[serde(alias = "Instances")]
            instances: Vec<Instance>,
        }
        #[derive(Debug, Deserialize)]
        struct Instance {
            #[serde(alias = "InstanceId")]
            instance_id: String,
            #[serde(alias = "PrivateIpAddress")]
            private_ip_address: Option<String>,
            #[serde(alias = "PublicIpAddress")]
            public_ip_address: Option<String>,
            #[serde(alias = "NetworkInterfaces")]
            network_interfaces: Vec<NetworkInterfaceResponse>,
        }
        #[derive(Debug, Deserialize)]
        struct NetworkInterfaceResponse {
            #[serde(alias = "PrivateIpAddress")]
            private_ip_address: String,
            #[serde(alias = "NetworkInterfaceId")]
            network_interface_id: String,
            #[serde(alias = "Attachment")]
            attachment: Attachment,
        }
        #[derive(Debug, Deserialize)]
        struct Attachment {
            #[serde(alias = "DeviceIndex")]
            device_index: i32,
        }

        #[derive(Debug, Deserialize)]
        struct DescribeInstancesResponse {
            #[serde(alias = "Reservations")]
            reservations: Vec<Reservation>,
        }
        #[derive(Debug, Deserialize)]
        struct Reservation {
            #[serde(alias = "Instances")]
            instances: Vec<Instance>,
        }

        let tag_specifications = self.tags.create_tags("instance", "aws-throwaway");
        let placement = format!(
            "AvailabilityZone={AZ},GroupName={}",
            self.placement_group_name
        );
        let user_data = base64::engine::general_purpose::STANDARD.encode(format!(
            r#"#!/bin/bash
sudo systemctl stop ssh
echo "{}" > /etc/ssh/ssh_host_ed25519_key.pub
echo "{}" > /etc/ssh/ssh_host_ed25519_key

echo "ClientAliveInterval 30" >> /etc/ssh/sshd_config
sudo systemctl start ssh
"#,
            self.host_public_key, self.host_private_key
        ));
        let block_device_mappings = format!(
            "DeviceName=/dev/sda1,Ebs={{DeleteOnTermination=true,VolumeSize={},VolumeType=gp2}}",
            definition.volume_size_gb
        );

        let mut command = vec![
            "ec2",
            "run-instances",
            "--tag-specifications",
            &tag_specifications,
            "--instance-type",
            definition.instance_type.as_str(),
            "--count",
            "1",
            "--placement",
            &placement,
            "--key-name",
            &self.keyname,
            "--image-id",
            &image_id,
            "--user-data",
            &user_data,
            "--block-device-mappings",
            &block_device_mappings,
        ];
        let value;
        if definition.network_interface_count == 1 {
            command.push("--subnet-id");
            command.push(&self.subnet_id);

            // if we specify a list of network interfaces we cannot specify an instance level security group
            command.push("--security-group-ids");
            command.push(&self.security_group_id);
        } else {
            command.push("--networking-interfaces");
            value = (0..definition.network_interface_count)
                .map(|i| {
                    format!(
                        "DeleteOnTermination=true,AssociatePublicIpAddress=false,DeviceIndex={i},Groups={},SubnetId={},Description={i}",
                        &self.security_group_id,
                        &self.subnet_id
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            command.push(&value);
        }

        let result: RunInstancesResponse = run_command(&command).await.unwrap();

        let instance = result.instances.first().unwrap();
        let primary_network_interface_id = &instance
            .network_interfaces
            .iter()
            .find(|x| x.attachment.device_index == 0)
            .unwrap()
            .network_interface_id;

        let network_interfaces: Vec<_> = instance
            .network_interfaces
            .iter()
            .map(|x| NetworkInterface {
                device_index: x.attachment.device_index,
                private_ipv4: x.private_ip_address.parse().unwrap(),
            })
            .collect();

        if let Some(elastic_ip) = &elastic_ip {
            let start = Instant::now();
            loop {
                let result: Result<Ignore> = run_command(&[
                    "ec2",
                    "associate-address",
                    "--allocation-id",
                    &elastic_ip.allocation_id,
                    "--network-interface-id",
                    primary_network_interface_id,
                ])
                .await;
                match result {
                    Ok(_) => {
                        break;
                    }
                    Err(err) => {
                        // It is expected to receive the following error if we attempt too early:
                        // `The pending-instance-running instance to which 'eni-***' is attached is not in a valid state for this operation`
                        if start.elapsed() > Duration::from_secs(120) {
                            panic!(
                                "Received error while associating address after 120s retrying: {err}",
                            );
                        } else {
                            tokio::time::sleep(Duration::from_secs(2)).await;
                        }
                    }
                }
            }
        }

        let mut public_ip: Option<IpAddr> = elastic_ip.map(|x| x.public_ip.parse().unwrap());
        let mut private_ip = None;

        let public_ip_expected = self.use_public_addresses || self.subnet_map_public_ip_on_launch;

        if public_ip_expected {
            tracing::info!("Waiting for instance private ip and public ip to be assigned");
        } else {
            tracing::info!("Waiting for instance private ip to be assigned");
        }
        while (public_ip_expected && public_ip.is_none()) || private_ip.is_none() {
            // There is no way the instance will be ready in 1 second,
            // so sleep before trying and then after all future attempts
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            let result: Result<DescribeInstancesResponse> = run_command(&[
                "ec2",
                "describe-instances",
                "--instance-ids",
                &instance.instance_id,
            ])
            .await;
            match result {
                Ok(instance) => {
                    for reservation in instance.reservations {
                        for instance in reservation.instances {
                            if public_ip.is_none() {
                                public_ip = instance.public_ip_address.map(|x| x.parse().unwrap());
                            }
                            private_ip = instance.private_ip_address.map(|x| x.parse().unwrap());
                        }
                    }
                }
                Err(err) => {
                    // InvalidInstanceID.NotFound can occur when we query too soon after creating the instance,
                    // so we need to retry when we hit that
                    if format!("{err}").contains("InvalidInstanceID.NotFound") {
                        panic!("Failed to describe instance {err:?}");
                    }
                }
            }
        }

        let private_ip = private_ip.unwrap();
        let connect_ip = if self.use_public_addresses {
            public_ip.unwrap()
        } else {
            private_ip
        };
        tracing::info!("created EC2 instance at public:{public_ip:?} private:{private_ip}");

        Ec2Instance::new(
            connect_ip,
            public_ip,
            private_ip,
            self.host_public_key_bytes.clone(),
            self.host_public_key.clone(),
            &self.client_private_key,
            network_interfaces,
        )
        .await
    }
}

async fn user_name() -> String {
    let GetUser::User { user_name } = run_command(&["iam", "get-user"]).await.unwrap();
    user_name
}

async fn run_command_empty_response(args: &[&str]) -> Result<()> {
    let text = run_command_string(args).await?;
    if text.is_empty() {
        Ok(())
    } else {
        Err(anyhow!(text))
    }
}

async fn run_command<T: for<'a> Deserialize<'a>>(args: &[&str]) -> Result<T> {
    let text = run_command_string(args).await?;
    match serde_json::from_str(&text) {
        Ok(value) => Ok(value),
        Err(err) => panic!("Failed to parse json with error {err}\ntext was:\n{text}"),
    }
}

async fn run_command_string(args: &[&str]) -> Result<String> {
    let output = tokio::process::Command::new("aws")
        .args(args)
        .output()
        .await
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    if !output.status.success() {
        Err(anyhow!(
            "command aws {args:?} failed:\nstdout:\n{stdout}\nstderr:\n{stderr}"
        ))
    } else {
        Ok(stdout)
    }
}

#[derive(serde::Deserialize)]
enum GetUser {
    User {
        #[serde(alias = "UserName")]
        user_name: String,
    },
}
