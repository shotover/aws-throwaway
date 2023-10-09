use aws_throwaway::{Aws, CleanupResources, Ec2InstanceDefinition, InstanceType};
use clap::Parser;
use std::str::FromStr;
use tracing_subscriber::EnvFilter;

const AWS_THROWAWAY_TAG: &str = "create-instance";

#[tokio::main]
async fn main() {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .init();

    let args = Args::parse();
    if args.cleanup {
        Aws::cleanup_resources_static(CleanupResources::WithAppTag(AWS_THROWAWAY_TAG.to_owned()))
            .await;
        println!("All AWS throwaway resources have been deleted")
    } else if let Some(instance_type) = args.instance_type {
        println!("Creating instance of type {instance_type}");

        let aws = Aws::new(CleanupResources::WithAppTag(AWS_THROWAWAY_TAG.to_owned())).await;
        let instance_type = InstanceType::from_str(&instance_type).unwrap();
        let network_interface_count = args.network_interfaces;
        let instance = aws
            .create_ec2_instance(
                Ec2InstanceDefinition::new(instance_type)
                    .volume_size_gigabytes(20)
                    .network_interface_count(network_interface_count)
                    .os(args.instance_os.to_aws()),
            )
            .await;

        let result = instance.ssh().shell("lsb_release -a").await;
        println!("Created instance running:\n{}", result.stdout);

        println!(
            "Run the following to ssh into it:\n{}",
            instance.ssh_instructions()
        );
    } else {
        println!("Need to specify either --cleanup or --instance-type")
    }
}

#[derive(Parser, Clone)]
#[clap()]
pub struct Args {
    /// e.g. --instance-type t2.micro
    #[clap(long)]
    pub instance_type: Option<String>,

    #[clap(long, default_value_t = 1)]
    pub network_interfaces: u32,

    #[clap(long)]
    pub instance_os: InstanceOs,

    #[clap(long)]
    pub cleanup: bool,
}

#[derive(clap::ValueEnum, Clone, Copy)]
pub enum InstanceOs {
    Ubuntu20_04,
    Ubuntu22_04,
}

impl InstanceOs {
    fn to_aws(self) -> aws_throwaway::InstanceOs {
        match self {
            InstanceOs::Ubuntu20_04 => aws_throwaway::InstanceOs::Ubuntu20_04,
            InstanceOs::Ubuntu22_04 => aws_throwaway::InstanceOs::Ubuntu22_04,
        }
    }
}
