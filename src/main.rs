use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use dotenv::dotenv;
use prometheus::{Encoder, Gauge, Opts, Registry};
use serde::{Deserialize, Serialize};
use warp::Filter;


#[derive(Serialize, Deserialize, Debug)]
struct ShellyStatus {
    id: i32,
    source: String,
    output: bool,
    apower: f32,
    voltage: f32,
    current: f32,
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Setup all the env vars
    let web_host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let web_port = env::var("PORT").unwrap_or_else(|_| "9000".to_string()).parse::<u16>()?;
    let shelly_host = env::var("SHELLY_HOST").expect("SHELLY_HOST environment variable is required.");
    let shelly_port = env::var("SHELLY_DEVICE_ID").unwrap_or_else(|_| "0".to_string());

    // and then the addresses
    let shelly_url = format!("http://{}/rpc/Switch.GetStatus?id={}", shelly_host, shelly_port);
    let web_addr: SocketAddr = format!("{}:{}", web_host, web_port).parse().expect("Failed to create a socket address!");

    // Create a new registry
    let registry = Arc::new(Registry::new());

    // Create a gauge for power usage
    let power_gauge = Gauge::with_opts(Opts::new(
        "shelly_power_watts",
        "Current power usage in watts"
    )).unwrap();

    // Register the gauge
    registry.register(Box::new(power_gauge.clone())).unwrap();

    // Create metrics endpoint
    let metrics_route = {
        // The clones inside the warp::path are going to move these variables into it
        // so we need to copy them prior
        let power_gauge = power_gauge.clone();
        let registry = registry.clone();
        let shelly_url = shelly_url.clone();

        warp::path!("metrics").then(move || {
            // And then we copy them again which ensures our references are thread-safe
            let power_gauge = power_gauge.clone();
            let registry = registry.clone();
            let shelly_url = shelly_url.clone();

            async move {
                // Grab the current value off the Shelly device
                match reqwest::get(shelly_url).await {
                    Ok(r) => {
                        if let Ok(body) = r.json::<ShellyStatus>().await {
                            power_gauge.set(body.apower as f64);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error fetching Shelly status: {}", e);
                    }
                }

                // Return metrics
                let encoder = prometheus::TextEncoder::new();
                let mut buffer = Vec::new();
                encoder.encode(&registry.gather(), &mut buffer).unwrap();
                String::from_utf8(buffer).unwrap()
            }
        })
    };

    println!("Starting metrics server on {} monitoring Shelly device at {}", web_addr, shelly_url);

    // Wait for ctrl-c/stop
    let (_, server) = warp::serve(metrics_route)
        .bind_with_graceful_shutdown(web_addr, async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen for ctrl-c");
            println!("\nShutdown signal received, shutting down...");
        });

    // Run the server
    server.await;
    Ok(())
}
