use reqwest;
use serde::Deserialize;
use dialoguer::Select;
use std::{error, process};

use crate::ForecastArgs;

#[derive(Deserialize)]
pub struct GeolocationResponse {
    pub results: Vec<Locations>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Locations {
    #[serde(alias = "name")]
    pub city: String,
    #[serde(alias = "admin1")]
    pub state: String,
    pub latitude: f64,
    pub longitude: f64,
}

fn get_matching_locations(location: &String) -> Result<Vec<Locations>, Box<dyn error::Error>> {
    let url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}",
        location
    );

    let response = reqwest::blocking::get(url)?.json::<GeolocationResponse>()?;
    let filtered_results = response
        .results
        .into_iter()
        .filter(|loc| loc.city.to_lowercase() == location.to_lowercase())
        .collect();

    Ok(filtered_results)
}

pub fn get_location(forecast: &ForecastArgs) -> Result<Locations, Box<dyn error::Error>> {
    let matching_locations = get_matching_locations(&forecast.city)?;

            let location_options: Vec<String> = matching_locations.iter().map(|location|
                 format!("{}, {}", location.city, location.state)
            ).collect();

    let mut selection = 0;
    if matching_locations.is_empty() {
        process::exit(1);
    } else if matching_locations.len() > 1 {
        selection = Select::new()
            .with_prompt("Multiple cities found please pick one")
            .default(0)
            .items(&location_options)
            .interact()?;
    }

    Ok(matching_locations[selection].clone())
}
