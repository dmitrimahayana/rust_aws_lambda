use rusoto_core::{Region, RusotoError};
use rusoto_s3::{S3Client, S3, GetObjectRequest};
use serde_json::Value;
use std::env;
use dotenv::dotenv;
use std::error::Error;
use tokio::io::AsyncReadExt; // Import the trait
// use std::io::Read;

async fn s3_config_download(bucket: &str, key: &str) -> Result<Value, Box<dyn Error>> {
    let client = S3Client::new(Region::default());

    let get_req = GetObjectRequest {
        bucket: bucket.to_string(),
        key: key.to_string(),
        ..Default::default()
    };

    let result = client.get_object(get_req).await?;
    let stream = result.body.ok_or("No body in response")?;
    let mut body = Vec::new();
    stream.into_async_read().read_to_end(&mut body).await?;

    let data = String::from_utf8(body)?;
    let json_data: Value = serde_json::from_str(&data)?;

    Ok(json_data)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the `.env` file
    dotenv().ok();

    let sm_configuration_bucket: String = env::var("SM_CONFIGURATION_BUCKET").expect("SM_CONFIGURATION_BUCKET environment variable is not set");
    let sm_client_id = env::var("SM_CLIENT").expect("SM_CLIENT environment variable is not set");
    let config_key = format!("{}/config.json", sm_client_id);

    match s3_config_download(&sm_configuration_bucket, &config_key).await {
        Ok(customer_config) => {
            // println!("Customer config: {:?}", customer_config);
            if let Some(rds_connection) = customer_config.get("rds_connection") {
                if let Some(host) = rds_connection.get("host").and_then(Value::as_str) {
                    println!("RDS Host: {}", host);
                }
                if let Some(password) = rds_connection.get("password").and_then(Value::as_str) {
                    println!("RDS Password: {}", password);
                }
                if let Some(port) = rds_connection.get("port").and_then(Value::as_u64) {
                    println!("RDS Port: {}", port);
                }
                if let Some(user) = rds_connection.get("user").and_then(Value::as_str) {
                    println!("RDS User: {}", user);
                }
            } else {
                println!("rds_connection not found in customer config");
            }
        }
        Err(err) => {
            eprintln!("Failed to download config: {}", err);
            return Err(err);
        }
    }

    Ok(())
}