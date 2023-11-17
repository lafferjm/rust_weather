use anyhow;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GeolocationResponse {
    pub results: Vec<Locations>,
}

#[derive(Deserialize)]
pub struct Locations {
    #[serde(alias = "name")]
    pub city: String,
    #[serde(alias = "admin1")]
    pub state: String,
    pub latitude: f64,
    pub longitude: f64,
}

pub fn get_matching_locations(location: &String) -> Result<Vec<Locations>, anyhow::Error> {
    let url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}",
        location
    );

    let response = reqwest::blocking::get(&url)?.json::<GeolocationResponse>()?;
    let filtered_results = response
        .results
        .into_iter()
        .filter(|loc| loc.city.to_lowercase() == location.to_lowercase())
        .collect();

    return Ok(filtered_results);
}
