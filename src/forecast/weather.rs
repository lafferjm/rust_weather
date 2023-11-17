use reqwest;
use std::error;

use serde::Deserialize;
use tabled::{Table, Tabled};

use crate::geolocation;

#[derive(Deserialize)]
struct ForecastResponse {
    daily: DailyForecastResponse,
}

#[derive(Deserialize)]
struct DailyForecastResponse {
    time: Vec<String>,
    #[serde(alias = "temperature_2m_min")]
    min_temp: Vec<f64>,
    #[serde(alias = "temperature_2m_max")]
    max_temp: Vec<f64>,
}

#[derive(Tabled)]
struct DailyForecastDisplay<'a> {
    #[tabled(rename = "Day")]
    day: &'a str,
    #[tabled(rename = "Min Temp")]
    min_temp: &'a f64,
    #[tabled(rename = "Max Temp")]
    max_temp: &'a f64,
}

fn get_forecast(
    location: &geolocation::Locations,
) -> Result<DailyForecastResponse, Box<dyn error::Error>> {
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&temperature_unit=fahrenheit&daily=temperature_2m_max,temperature_2m_min",
                      location.latitude, location.longitude,);

    let response = reqwest::blocking::get(&url)?.json::<ForecastResponse>()?;

    return Ok(response.daily);
}

pub fn display_forecast(location: &geolocation::Locations) -> Result<String, Box<dyn error::Error>> {
    let forecast = get_forecast(location)?;

    let daily: Vec<DailyForecastDisplay> = forecast
    .time
    .iter()
    .zip(forecast.min_temp.iter().zip(forecast.max_temp.iter()))
    .map(|(day, (min_temp, max_temp))| DailyForecastDisplay {
        day,
        min_temp,
        max_temp,
    })
    .collect();

    return Ok(Table::new(&daily).to_string());
}
