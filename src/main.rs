mod forecast;

use clap::{Args, Parser, Subcommand};
use dialoguer::Select;
use forecast::{geolocation, weather};
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

            let mut items = Vec::new();

            for location in matching_locations.iter() {
                let city_state = format!("{}, {}", location.city, location.state);
                items.push(city_state);
            }

            let mut selection = 0;
            if matching_locations.len() == 0 {
                println!("Unable to locate: {}", forecast.city);
                process::exit(1);
            } else if matching_locations.len() > 1 {
                selection = Select::new()
                    .with_prompt("Multiple cities found please pick one?")
                    .default(0)
                    .items(&items)
                    .interact()
                    .unwrap();
            }

            let forecast =
                weather::get_forecast(&matching_locations[selection]).unwrap_or_else(|_| {
                    eprintln!("Error getting the forecast");
                    process::exit(1);
                });

            let display = weather::display_forecast(&forecast);
            println!("{}", display);
        }
    }
}
