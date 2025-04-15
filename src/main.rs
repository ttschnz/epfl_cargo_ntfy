mod api;
mod model;
mod ntfy;

use api::fetch_bikes;
use std::{env::var, thread::sleep, time::Duration};

#[derive(Clone, Debug)]
struct Args {
    scanning_interval: u64,
}

impl Args {
    fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let scanning_interval = var("scanning_interval")
            .ok()
            .and_then(|interval_str| interval_str.parse::<u64>().ok())
            .unwrap_or((60 * 60) as u64); // default to 1hr

        Ok(Self { scanning_interval })
    }
}

fn main() {
    let args = Args::from_env().unwrap();

    loop {
        let bikes = fetch_bikes((46.536678, 6.590684), (46.506217, 6.534188)).unwrap();
        bikes.hubs.iter().for_each(|hub| {
            if hub.available_vehicles_count > 0 {
                let _ = ntfy::send_notification(
                    "epfl_cargo_bikes",
                    &format!(
                        "{} bikes are available at {}",
                        hub.available_vehicles_count, hub.name
                    ),
                );
            }
        });
        sleep(Duration::from_secs(args.scanning_interval));
    }
}
