use reqwest;
use std::error;

use serde::Deserialize;
use tabled::{Table, Tabled};

use crate::geolocation;

#[derive(Deserialize)]
pub struct ForecastResponse {
    pub daily: DailyForecastResponse,
}

#[derive(Deserialize)]
pub struct DailyForecastResponse {
    pub time: Vec<String>,
    #[serde(alias = "temperature_2m_min")]
    pub min_temp: Vec<f64>,
    #[serde(alias = "temperature_2m_max")]
    pub max_temp: Vec<f64>,
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

pub fn get_forecast(
    location: &geolocation::Locations,
) -> Result<DailyForecastResponse, Box<dyn error::Error>> {
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&temperature_unit=fahrenheit&daily=temperature_2m_max,temperature_2m_min",
                      location.latitude, location.longitude,);

    let response = reqwest::blocking::get(&url)?.json::<ForecastResponse>()?;

    return Ok(response.daily);
}

pub fn display_forecast(forecast: &DailyForecastResponse) -> String {
    let mut daily = Vec::new();
    for i in 0..forecast.time.len() {
        let forecast_display = DailyForecastDisplay {
            day: &forecast.time[i],
            min_temp: &forecast.min_temp[i],
            max_temp: &forecast.max_temp[i],
        };
        daily.push(forecast_display);
    }

    return Table::new(&daily).to_string();
}
