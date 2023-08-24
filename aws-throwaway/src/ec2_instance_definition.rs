use aws_sdk_ec2::types::InstanceType;

/// Defines an instance that can be launched via [`Aws::create_ec2_instance`]
pub struct Ec2InstanceDefinition {
    pub(crate) instance_type: InstanceType,
    pub(crate) volume_size_gb: u32,
}

impl Ec2InstanceDefinition {
    /// Start defining an instance with the specified instance type
    pub fn new(instance_type: InstanceType) -> Self {
        Ec2InstanceDefinition {
            instance_type,
            volume_size_gb: 8,
        }
    }

    // Set instance to have a root volume of the specified size.
    // Defaults to 8GB.
    pub fn volume_size_gigabytes(mut self, size_gb: u32) -> Self {
        self.volume_size_gb = size_gb;
        self
    }
}
