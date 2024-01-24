#[cfg(not(feature = "use_sdk"))]
pub(crate) mod cli;
#[cfg(feature = "use_sdk")]
pub(crate) mod sdk;

#[cfg(not(feature = "use_sdk"))]
pub use cli::{Aws, InstanceType, PlacementStrategy};

#[cfg(feature = "use_sdk")]
pub use aws_sdk_ec2::types::{InstanceType, PlacementStrategy};
#[cfg(feature = "use_sdk")]
pub use sdk::aws::Aws;
