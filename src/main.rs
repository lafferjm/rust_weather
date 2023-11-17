use clap::{Args, Parser, Subcommand};

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
            println!("You entered city: {:?}", forecast.city)
        }
    }
}
