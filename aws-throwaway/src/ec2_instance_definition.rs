use crate::InstanceType;

/// Defines an instance that can be launched via [`crate::Aws::create_ec2_instance`]
pub struct Ec2InstanceDefinition {
    pub(crate) instance_type: InstanceType,
    pub(crate) volume_size_gb: u32,
    pub(crate) network_interface_count: u32,
    pub(crate) os: InstanceOs,
    pub(crate) ami: Option<String>,
}

impl Ec2InstanceDefinition {
    /// Start defining an instance with the specified instance type
    pub fn new(instance_type: InstanceType) -> Self {
        Ec2InstanceDefinition {
            instance_type,
            volume_size_gb: 8,
            network_interface_count: 1,
            os: InstanceOs::Ubuntu22_04,
            ami: None,
        }
    }

    // Set instance to have a root volume of the specified size.
    // Defaults to 8GB.
    pub fn volume_size_gigabytes(mut self, size_gb: u32) -> Self {
        self.volume_size_gb = size_gb;
        self
    }

    /// Sets the amount of network interfaces to use on this instance.
    /// Defaults to 1
    ///
    /// Setting this to a value other than 1 will result in the creation of an elastic ip to point at your instance.
    /// This is an unfortunate requirement of AWS ECS, instances with multiple network interfaces do not get the automatically assigned ipv4 address given to instances with 1 network interface.
    /// For most users there is a hard limit of 5 elastic ip addresses allowed at one time.
    pub fn network_interface_count(mut self, count: u32) -> Self {
        self.network_interface_count = count;
        self
    }

    // Set the OS used by the instance.
    // Defaults to `Ubuntu 22.04`
    pub fn os(mut self, os: InstanceOs) -> Self {
        self.os = os;
        self
    }

    /// Override the AMI used.
    /// When used together with the `os` setting, the os setting is used to determine how to configure the instance while the specified AMI is used as the instances image.
    /// Defaults to None which indicates that the appropriate AMI should be looked up via SSM.
    ///
    /// This option is useful when you have custom variation of the configured OS or if your user does not have access to SSM.
    /// AMI's are region specific so be careful in picking your AMI.
    pub fn override_ami(mut self, ami: Option<String>) -> Self {
        self.ami = ami;
        self
    }
}

/// aws-throwaway needs to manually support each OS, so the only OS's you can use are those listed in this enum.
/// Support for more (similar) OS's is welcome via pull request.
#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum InstanceOs {
    Ubuntu20_04,
    Ubuntu22_04,
}
