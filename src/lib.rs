mod forecast;

use std::process;

use clap::{Args, Parser, Subcommand};
use forecast::{geolocation, weather};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get the forecast for a city
    Forecast(ForecastArgs),
}

#[derive(Args)]
pub struct ForecastArgs {
    #[arg(short, long)]
    pub city: String,
}

pub fn get_args() -> Cli {
    

    Cli::parse()
}

pub fn run(cli: &Cli) {
    match &cli.command {
        Commands::Forecast(forecast) => {
            let location = geolocation::get_location(forecast).unwrap_or_else(|_| {
                eprintln!("Error getting the locations");
                process::exit(1);
            });

            let forecast = weather::display_forecast(&location).unwrap_or_else(|_| {
                eprintln!("Error getting the forecast");
                process::exit(1);
            });

            println!("{}", forecast);
        }
    }
}