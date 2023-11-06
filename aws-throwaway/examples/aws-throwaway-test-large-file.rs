use aws_throwaway::{Aws, CleanupResources, Ec2InstanceDefinition, InstanceType};
use std::{path::Path, time::Instant};
use tracing_subscriber::EnvFilter;

const FILE_LEN: usize = 1024 * 1024 * 100; // 100MB

#[tokio::main]
async fn main() {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .init();

    let aws = Aws::builder(CleanupResources::AllResources).build().await;
    let instance = aws
        .create_ec2_instance(Ec2InstanceDefinition::new(InstanceType::T2Micro))
        .await;

    let start = Instant::now();
    std::fs::write("some_local_file", vec![0; FILE_LEN]).unwrap(); // create 100MB file
    println!("Time to create 100MB file locally {:?}", start.elapsed());

    let start = Instant::now();
    instance
        .ssh()
        .push_file(Path::new("some_local_file"), Path::new("some_remote_file"))
        .await;
    println!("Time to push 100MB file {:?}", start.elapsed());

    let start = Instant::now();
    instance
        .ssh()
        .pull_file(Path::new("some_remote_file"), Path::new("some_local_file"))
        .await;
    println!("Time to pull 100MB file {:?}", start.elapsed());

    assert_eq!(std::fs::read("some_local_file").unwrap().len(), FILE_LEN);
    std::fs::remove_file("some_local_file").unwrap();

    aws.cleanup_resources().await;
    println!("\nAll AWS throwaway resources have been deleted")
}
