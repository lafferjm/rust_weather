use anyhow;
use reqwest;

use serde::Deserialize;

use crate::geolocation;

#[derive(Deserialize)]
pub struct ForecastResponse {
    pub daily: DailyForecast,
}

#[derive(Deserialize)]
pub struct DailyForecast {
    pub time: Vec<String>,
    #[serde(alias = "temperature_2m_min")]
    pub min_temp: Vec<f64>,
    #[serde(alias = "temperature_2m_max")]
    pub max_temp: Vec<f64>,
}

pub fn get_forecast(location: &geolocation::Locations) -> Result<DailyForecast, anyhow::Error> {
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&temperature_unit=fahrenheit&daily=temperature_2m_max,temperature_2m_min",
                      location.latitude, location.longitude,);

    let response = reqwest::blocking::get(&url)?.json::<ForecastResponse>()?;

    return Ok(response.daily);
}
