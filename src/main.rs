use rusoto_core::{Region};
use chrono::{NaiveDateTime, Duration};
use rusoto_s3::{S3Client, S3, GetObjectRequest};
use serde_json::Value;
use std::env;
use dotenv::dotenv;
use std::error::Error;
use tokio::io::AsyncReadExt; // Import the trait
use tokio_postgres::{Client, NoTls};
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

async fn connect_db(host: &str, user: &str, password: &str, database: &str, port: u16) -> Result<Client, Box<dyn Error>> {
    let connection_string = format!(
        "host={} user={} password={} dbname={} port={}",
        host, user, password, database, port
    );

    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

    // Spawn the connection to run in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    Ok(client)
}

async fn perform_query(client: &Client) -> Result<(), Box<dyn Error>> {
    // Perform a query
    let rows = client.query("SELECT post_id, media_type, TO_CHAR(date_posted, 'YYYY-MM-DD HH24:MI:SS') AS date_posted FROM analytics.aggregation_posts LIMIT 25", &[]).await?;

    // Process the query result
    for row in rows {
        let post_id: String = row.get(0);
        let media_type: String = row.get(1);
        let date_posted: String = row.get(2);
        println!("post_id: {} media_type: {} date_posted: {}", post_id, media_type, date_posted);

        let naive_datetime = NaiveDateTime::parse_from_str(&date_posted, "%Y-%m-%d %H:%M:%S")?;
        let new_naive_datetime = naive_datetime + Duration::days(7);
        // Print the new NaiveDateTime as a string
        let new_naive_datetime_str = new_naive_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("New NaiveDateTime: {}", new_naive_datetime_str);
    }

    Ok(())
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
                if let (Some(host), Some(user), Some(password), Some(port), Some(database)) = (
                    rds_connection.get("host").and_then(Value::as_str),
                    rds_connection.get("user").and_then(Value::as_str),
                    rds_connection.get("password").and_then(Value::as_str),
                    rds_connection.get("port").and_then(Value::as_u64),
                    rds_connection.get("database").and_then(Value::as_str),) {
                    println!("RDS Host: {}", host);
                    println!("RDS User: {}", user);
                    println!("RDS Password: {}", password);
                    println!("RDS Port: {}", port);
                    println!("RDS DB: {}", database);
                    let client = connect_db(host, user, password, database, port as u16).await?;
                    perform_query(&client).await?;
                } else {
                    println!("Incomplete RDS configuration");
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