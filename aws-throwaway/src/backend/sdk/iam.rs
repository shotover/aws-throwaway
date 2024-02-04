use anyhow::Result;
use aws_config::SdkConfig;

pub async fn user_name(config: &SdkConfig) -> String {
    match iam_user_name(config).await {
        Ok(name) => name,
        Err(err) => {
            tracing::debug!("Failed to run iam get-user, falling back to STS, error was: {err:?}");
            sts_user_id(config).await
        }
    }
}

pub async fn iam_user_name(config: &SdkConfig) -> Result<String> {
    let client = aws_sdk_iam::Client::new(config);
    Ok(client
        .get_user()
        .send()
        .await
        .map_err(|e| e.into_service_error())?
        .user()
        .unwrap()
        .user_name()
        .to_string())
}

pub async fn sts_user_id(config: &SdkConfig) -> String {
    let client = aws_sdk_sts::Client::new(config);
    client
        .get_caller_identity()
        .send()
        .await
        .map_err(|e| e.into_service_error())
        .unwrap()
        .user_id()
        .unwrap()
        .to_string()
}
