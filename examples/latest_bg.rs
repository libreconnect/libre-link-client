use librelink_client::client::{Credentials, LibreLinkClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = env::var("LIBRELINK_USERNAME")
        .expect("LIBRELINK_USERNAME environment variable must be set");
    let password = env::var("LIBRELINK_PASSWORD")
        .expect("LIBRELINK_PASSWORD environment variable must be set");

    println!("Connecting to LibreLink...");

    let client = LibreLinkClient::new(
        Credentials { username, password },
        None,
    )
    .await?;

    let connections = client.get_connections().await?;

    if connections.data.is_empty() {
        println!("No connections found. Make sure you have a linked patient.");
        return Ok(());
    }

    let connection = &connections.data[0];
    let measurement = &connection.glucose_measurement;

    println!("Patient: {} {}", connection.first_name, connection.last_name);
    println!("Latest BG: {} mg/dL ({:.1} mmol/L)", measurement.value_in_mg_per_dl, measurement.value);
    println!("Timestamp: {}", measurement.timestamp);

    if measurement.is_high {
        println!("⚠️  HIGH");
    } else if measurement.is_low {
        println!("⚠️  LOW");
    } else {
        println!("✅ In range");
    }

    Ok(())
}
