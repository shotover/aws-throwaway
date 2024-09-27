use aws_throwaway::{Aws, CleanupResources, Ec2InstanceDefinition, InstanceType};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .init();

    println!("Creating instances");
    let aws = Aws::builder(CleanupResources::AllResources)
        .use_ingress_restriction(aws_throwaway::IngressRestriction::LocalPublicAddress)
        .build()
        .await;
    let (instance1, instance2) = tokio::join!(
        aws.create_ec2_instance(Ec2InstanceDefinition::new(InstanceType::T2Small)),
        aws.create_ec2_instance(
            Ec2InstanceDefinition::new(InstanceType::T2Small).network_interface_count(2)
        )
    );

    println!("pinging instance2 from instance1");
    let ip = instance2.private_ip();
    let output = instance1.ssh().shell(&format!("ping {ip} -c 4")).await;
    println!("{output}");

    aws.cleanup_resources().await;
    println!("\nAll AWS throwaway resources have been deleted")
}
