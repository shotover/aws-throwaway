use aws_throwaway::{Aws, CleanupResources, Ec2InstanceDefinition, InstanceType};
use std::path::Path;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .init();

    let aws = Aws::builder(CleanupResources::AllResources)
        .use_ingress_restriction(aws_throwaway::IngressRestriction::LocalPublicAddress)
        .build()
        .await;
    let instance = aws
        .create_ec2_instance(Ec2InstanceDefinition::new(InstanceType::T2Micro))
        .await;

    instance
        .ssh()
        .shell("echo 'some string' > some_remote_file")
        .await;

    let result = instance.ssh().shell("xxd some_remote_file").await;
    println!("The bytes of the remote file:\n{}", result.stdout);

    // download a file and assert on its contents
    instance
        .ssh()
        .pull_file(Path::new("some_remote_file"), Path::new("some_local_file"))
        .await;
    assert_eq!(
        std::fs::read_to_string("some_local_file").unwrap(),
        "some string\n"
    );
    std::fs::remove_file("some_local_file").unwrap();

    instance
        .ssh()
        .push_file(Path::new("readme.md"), Path::new("readme.md"))
        .await;
    let mut receiver = instance.ssh().shell_stdout_lines("cat readme.md").await;
    println!();
    while let Some(line) = receiver.recv().await {
        let line = line.unwrap();
        println!("Received: {line}");
    }

    aws.cleanup_resources().await;
    println!("\nAll AWS throwaway resources have been deleted")
}
