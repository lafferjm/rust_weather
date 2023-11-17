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
    let cli = Cli::parse();

    return cli;
}

pub fn run(cli: &Cli) {
    match &cli.command {
        Commands::Forecast(forecast) => {
            let jank = geolocation::get_location(forecast);
            // let forecast =
            //     weather::get_forecast(&matching_locations[selection]).unwrap_or_else(|_| {
            //         eprintln!("Error getting the forecast");
            //         process::exit(1);
            //     });

            // let display = weather::display_forecast(&forecast);
            // println!("{}", display);
        }
    }
}