use librelink_client::client::{Credentials, LibreLinkClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = env::var("LIBRELINK_USERNAME")
        .expect("LIBRELINK_USERNAME environment variable must be set");
    let password = env::var("LIBRELINK_PASSWORD")
        .expect("LIBRELINK_PASSWORD environment variable must be set");
    let region = env::var("LIBRELINK_REGION").ok();

    println!("Connecting to LibreLink...");

    let mut client = LibreLinkClient::new(
        Credentials { username, password },
        region,
    )
    .await?;

    let connections = client.get_connections().await?;

    if connections.data.is_empty() {
        println!("No connections found. Make sure you have a linked patient.");
        return Ok(());
    }

    let connection = &connections.data[0];
    println!("Patient: {} {}\n", connection.first_name, connection.last_name);

    // Fetch the graph data (recent glucose readings)
    let graph = client.get_connection_graph(&connection.patient_id).await?;
    let readings = &graph.data.graph_data;

    if readings.is_empty() {
        println!("No graph data available.");
        return Ok(());
    }

    // Stats
    let values: Vec<i32> = readings.iter().map(|r| r.value_in_mg_per_dl).collect();
    let min = values.iter().min().unwrap();
    let max = values.iter().max().unwrap();
    let avg = values.iter().sum::<i32>() as f64 / values.len() as f64;
    let highs = readings.iter().filter(|r| r.is_high).count();
    let lows = readings.iter().filter(|r| r.is_low).count();
    let in_range = readings.len() - highs - lows;

    println!("=== Graph Summary ({} readings) ===", readings.len());
    println!("  Min:      {} mg/dL", min);
    println!("  Max:      {} mg/dL", max);
    println!("  Average:  {:.0} mg/dL", avg);
    println!("  In range: {} ({:.0}%)", in_range, in_range as f64 / readings.len() as f64 * 100.0);
    println!("  High:     {} ({:.0}%)", highs, highs as f64 / readings.len() as f64 * 100.0);
    println!("  Low:      {} ({:.0}%)", lows, lows as f64 / readings.len() as f64 * 100.0);

    println!("\n=== Recent Readings ===");
    println!("{:<22} {:>8} {:>10}  {}", "Timestamp", "mg/dL", "mmol/L", "Status");
    println!("{}", "-".repeat(55));

    for reading in readings.iter().rev().take(20) {
        let status = if reading.is_high {
            "⚠️  HIGH"
        } else if reading.is_low {
            "⚠️  LOW"
        } else {
            "✅"
        };

        println!(
            "{:<22} {:>8} {:>10.1}  {}",
            reading.timestamp,
            reading.value_in_mg_per_dl,
            reading.value,
            status,
        );
    }

    if readings.len() > 20 {
        println!("... and {} more readings", readings.len() - 20);
    }

    // Active sensor info
    if let Some(sensor) = graph.data.active_sensors.first() {
        println!("\n=== Active Sensor ===");
        println!("  Serial: {}", sensor.sensor.sn);
        println!("  Device: {} (v{})", sensor.device.did, sensor.device.v);
    }

    Ok(())
}
