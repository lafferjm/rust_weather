mod forecast;

use clap::{Args, Parser, Subcommand};
use forecast::geolocation;
use std::process;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get the forecast for a city
    Forecast(ForecastArgs),
}

#[derive(Args)]
struct ForecastArgs {
    city: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Forecast(forecast) => {
            let matching_locations = geolocation::get_matching_locations(&forecast.city)
                .unwrap_or_else(|_| {
                    eprintln!("Error searching for location");
                    process::exit(1);
                });

            for location in matching_locations {
                println!("{:?}", location);
            }
        }
    }
}
