use crate::forecast::geolocation;
use clap::{Args, Parser, Subcommand};

pub mod forecast;

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
            geolocation::get_matching_locations(&forecast.city);
            // println!("You entered city: {:?}", forecast.city)
        }
    }
}
