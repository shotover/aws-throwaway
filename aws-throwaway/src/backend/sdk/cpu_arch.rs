use aws_sdk_ec2::types::InstanceType;

pub enum CpuArch {
    X86_64,
    Aarch64,
}

impl CpuArch {
    pub fn get_ubuntu_arch_identifier(&self) -> &'static str {
        match self {
            CpuArch::X86_64 => "amd64",
            CpuArch::Aarch64 => "arm64",
        }
    }
}

pub fn get_arch_of_instance_type(instance_type: InstanceType) -> CpuArch {
    // Instance names look something like:
    // type + revision_number + subtypes + '.' + size
    // So say for example `Im4gn.large` would be split into:
    // type = "Im"
    // revision_number = 4
    // subtypes = "gn"
    // size = "large"
    //
    // The 'g' character existing in subtypes indicates that the instance type is a gravitron aka arm instance.
    // We can check for the existence of 'g' to determine if we are aarch64 or x86_64
    // This is a bit hacky because this format is not explicitly documented anywhere but the instance type naming does consistently follow this pattern.
    let mut reached_revision_number = false;
    for c in instance_type.as_str().chars() {
        if !reached_revision_number {
            if c.is_ascii_digit() {
                reached_revision_number = true;
            }
        } else if c == '.' {
            return CpuArch::X86_64;
        } else if c == 'g' {
            return CpuArch::Aarch64;
        }
    }
    unreachable!("Cannot parse instance type: {instance_type:?}")
}
