mod backend;
mod ec2_instance;
mod ec2_instance_definition;
mod ssh;

use std::net::IpAddr;

pub use backend::{Aws, InstanceType, PlacementStrategy};
pub use ec2_instance::{Ec2Instance, NetworkInterface};
pub use ec2_instance_definition::{Ec2InstanceDefinition, InstanceOs};

// include a magic number in the keyname to avoid collisions
// This can never change or we may fail to cleanup resources.
const USER_TAG_NAME: &str = "aws-throwaway-23c2d22c-d929-43fc-b2a4-c1c72f0b733f:user";
const APP_TAG_NAME: &str = "aws-throwaway-23c2d22c-d929-43fc-b2a4-c1c72f0b733f:app";

pub struct AwsBuilder {
    cleanup: CleanupResources,
    use_public_addresses: bool,
    ingress_restriction: IngressRestriction,
    vpc_id: Option<String>,
    az_name: Option<String>,
    subnet_id: Option<String>,
    placement_strategy: PlacementStrategy,
    security_group_id: Option<String>,
    expose_ports_to_internet: Vec<u16>,
}

/// The default configuration will succeed for an AMI user with sufficient access and unmodified default vpcs/subnets
/// Consider altering the configuration if:
/// * you want to reduce the amount of access required by the user
/// * you want to connect directly from within the VPC
/// * you have already created a specific VPC, subnet or security group that you want aws-throwaway to make use of.
///
/// All resources will be created in us-east-1.
/// This is hardcoded so that aws-throwaway only has to look into one region when cleaning up.
/// All instances are created in a single spread placement group in a single AZ to ensure consistent latency between instances.
// TODO: document minimum required access for default configuration.
impl AwsBuilder {
    fn new(cleanup: CleanupResources) -> Self {
        AwsBuilder {
            cleanup,
            use_public_addresses: true,
            ingress_restriction: IngressRestriction::NoRestrictions,
            vpc_id: None,
            az_name: None,
            subnet_id: None,
            // refer to: https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html
            // I believe Spread is the best default since it is the easiest for amazon to fulfill and gives the most realistic results in benchmarks.
            placement_strategy: PlacementStrategy::Spread,
            security_group_id: None,
            expose_ports_to_internet: vec![],
        }
    }

    /// When set to:
    /// * true => aws-throwaway will connect to the public ip of the instances that it creates.
    ///     + The subnet must have the property MapPublicIpOnLaunch set to true (the unmodified default subnet meets this requirement)
    ///     + Elastic IPs will be created for instances with multiple network interfaces because AWS does not assign a public IP in that scenario
    /// * false => aws-throwaway will connect to the private ip of the instances that it creates.
    ///     + aws-throwaway must be running on a machine within the VPC used by aws-throwaaway or a VPN must be used to connect to the VPC or another similar setup.
    ///
    /// If the subnet used has MapPublicIpOnLaunch=true then all instances will be publically accessible regardless of this use_public_addresses field.
    ///
    /// The default is `true`.
    pub fn use_public_addresses(mut self, use_public_addresses: bool) -> Self {
        self.use_public_addresses = use_public_addresses;
        self
    }

    pub fn use_ingress_restriction(mut self, ingress_restriction: IngressRestriction) -> Self {
        self.ingress_restriction = ingress_restriction;
        self
    }

    /// * Some(_) => All resources will go into the specified vpc
    /// * None => All resources will go into the default vpc
    ///
    /// The default is `None`
    pub fn use_vpc_id(mut self, vpc_id: Option<String>) -> Self {
        self.vpc_id = vpc_id;
        self
    }

    /// * Some(_) => All resources will go into the specified AZ
    /// * None => All resources will go into the default AZ (us-east-1c)
    ///
    /// The default is `None`
    pub fn use_az(mut self, az_name: Option<String>) -> Self {
        self.az_name = az_name;
        self
    }

    /// * Some(_) => All instances will go into the specified subnet
    /// * None => All instances will go into the default subnet for the specified or default vpc
    ///
    /// The default is `None`
    pub fn use_subnet_id(mut self, subnet_id: Option<String>) -> Self {
        self.subnet_id = subnet_id;
        self
    }

    /// All EC2 instances are created within a single placement group with the specified strategy.
    ///
    /// The default is `PlacementStrategy::Spread`
    pub fn use_placement_strategy(mut self, placement_strategy: PlacementStrategy) -> Self {
        self.placement_strategy = placement_strategy;
        self
    }

    /// * Some(_) => All instances will use the specified security group
    /// * None => A single security group will be created for all instances to use. It will allow:
    ///      + ssh traffic in from the internet
    ///      + all traffic out to the internet
    ///      + all traffic in+out between instances in the security group, i.e. all ec2 instances created by this [`Aws`] instance
    ///
    /// The default is `None`
    pub fn use_security_group_id(mut self, security_group_id: Option<String>) -> Self {
        self.security_group_id = security_group_id;
        self
    }

    /// Adds the provided ports as allowing traffic in+out to internet in the automatically generated security group.
    /// By default ingress is allowed from port 22 and this cannot be disabled.
    pub fn expose_ports_to_internet(mut self, ports: Vec<u16>) -> Self {
        self.expose_ports_to_internet = ports;
        self
    }

    /// Builds the Aws instance.
    ///
    /// Will panic if both `expose_ports_to_internet` and `use_security_group_id` are enabled.
    pub async fn build(self) -> Aws {
        if !self.expose_ports_to_internet.is_empty() && self.security_group_id.is_some() {
            panic!("Both `use_security_group_id` and `expose_ports_to_internet` are set. Ensure only one of these options is set.")
        }
        Aws::new(self).await
    }
}

/// Specify the cleanup process to use.
pub enum CleanupResources {
    /// Cleanup resources created by all [`Aws`] instances that use [`CleanupResources::WithAppTag`] of the same tag.
    /// It is highly reccomended that this tag is hardcoded, generating this tag could easily lead to forgotten resources.
    WithAppTag(String),
    /// Cleanup resources created by all [`Aws`] instances regardless of whether it was created via [`CleanupResources::AllResources`] or [`CleanupResources::WithAppTag`]
    AllResources,
}

/// Defines how to derive the ingress rules of the generated security group for external access.
///
/// Internal network traffic between instances created through aws-throwaway is always allowed,
/// regardless of the `IngressRestriction` value used.
///
/// These rules apply to the always enabled port 22 and any extra ports enabled by `AwsBuilder::expose_ports_to_internet`.
#[non_exhaustive]
pub enum IngressRestriction {
    /// Allow ingress from any machine on the internet.
    /// Many corporate environments will disallow this.
    NoRestrictions,
    /// Allow ingress only from the public IP address of the machine aws-throwaway is running on.
    /// Possibly slightly slower to startup, the public IP will be fetched from https://api.ipify.org in parallel to other work.
    LocalPublicAddress,
    // In the future we might add:
    //UseSpecificAddress(IpAddr)
}

impl IngressRestriction {
    async fn cidr_ip(&self) -> String {
        match self {
            IngressRestriction::NoRestrictions => "0.0.0.0/0".to_owned(),
            IngressRestriction::LocalPublicAddress => {
                let api = "https://api.ipify.org";
                let ip = reqwest::get(api).await.unwrap().text().await.unwrap();
                // roundtrip through IpAddr to ensure that we did in fact receive an IP.
                let ip: IpAddr = ip.parse().unwrap();
                format!("{ip}/32")
            }
        }
    }
}
