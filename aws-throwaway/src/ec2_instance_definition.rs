use aws_sdk_ec2::types::InstanceType;

/// Defines an instance that can be launched via [`Aws::create_ec2_instance`]
pub struct Ec2InstanceDefinition {
    pub(crate) instance_type: InstanceType,
    pub(crate) volume_size_gb: u32,
    pub(crate) os: InstanceOs,
}

impl Ec2InstanceDefinition {
    /// Start defining an instance with the specified instance type
    pub fn new(instance_type: InstanceType) -> Self {
        Ec2InstanceDefinition {
            instance_type,
            volume_size_gb: 8,
            os: InstanceOs::Ubuntu20_04,
        }
    }

    // Set instance to have a root volume of the specified size.
    // Defaults to 8GB.
    pub fn volume_size_gigabytes(mut self, size_gb: u32) -> Self {
        self.volume_size_gb = size_gb;
        self
    }

    // Set the OS used by the instance.
    // Defaults to `Ubuntu 22.04`
    pub fn os(mut self, os: InstanceOs) -> Self {
        self.os = os;
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
