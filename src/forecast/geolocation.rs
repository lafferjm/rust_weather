use anyhow;
use reqwest;

pub fn get_matching_locations(location: &String) -> Result<(), anyhow::Error> {
    let url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}",
        location
    );

    let body = reqwest::blocking::get(url)?.text()?;
    println!("body = {:?}", body);

    return Ok(());
}
