extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate prettytable;

use serde::{Deserialize, Serialize};
use std::io;
use prettytable::{Table, Row, Cell};

// Structs to deserialize weather data from API response
#[derive(Debug, Serialize, Deserialize)]
struct WeatherData {
    location: Location,
    current: Current,
}

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    name: String,
    country: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Current {
    temp_c: f32,
    feelslike_c: f32,
    condition: Condition,
    wind_kph: f32,
    wind_dir: String,
    pressure_mb: f32,
    humidity: u8,
    cloud: u8,
    air_quality: AirQuality,
}

#[derive(Debug, Serialize, Deserialize)]
struct Condition {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AirQuality {
    co: f32,
    no2: f32,
    o3: f32,
    so2: f32,
    pm2_5: f32,
    pm10: f32,
}

// Function to fetch weather data from WeatherAPI.com API
async fn fetch_weather(city: &str, api_key: &str) -> Result<WeatherData, reqwest::Error> {
    let url = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=yes", api_key, city);
    let response = reqwest::get(&url).await?.json::<WeatherData>().await?;

    Ok(response)
}

fn print_weather_data(weather_data: &WeatherData) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("City"),
        Cell::new(&weather_data.location.name),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Country"),
        Cell::new(&weather_data.location.country),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Temperature (°C)"),
        Cell::new(&weather_data.current.temp_c.to_string()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Feels Like (°C)"),
        Cell::new(&weather_data.current.feelslike_c.to_string()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Condition"),
        Cell::new(&weather_data.current.condition.text),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Wind Speed (km/h)"),
        Cell::new(&weather_data.current.wind_kph.to_string()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Wind Direction"),
        Cell::new(&weather_data.current.wind_dir),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Pressure (mb)"),
        Cell::new(&weather_data.current.pressure_mb.to_string()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Humidity (%)"),
        Cell::new(&weather_data.current.humidity.to_string()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Cloud Coverage (%)"),
        Cell::new(&weather_data.current.cloud.to_string()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("CO (µg/m³)"),
        Cell::new(&weather_data.current.air_quality.co.to_string()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("NO2 (µg/m³)"),
        Cell::new(&weather_data.current.air_quality.no2.to_string()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("O3 (µg/m³)"),
        Cell::new(&weather_data.current.air_quality.o3.to_string()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("SO2 (µg/m³)"),
        Cell::new(&weather_data.current.air_quality.so2.to_string()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("PM2.5 (µg/m³)"),
        Cell::new(&weather_data.current.air_quality.pm2_5.to_string()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("PM10 (µg/m³)"),
        Cell::new(&weather_data.current.air_quality.pm10.to_string()),
    ]));

    table.printstd();
}

#[tokio::main]
async fn main() {

    // Get the WeatherAPI.com API key from the environment
    let api_key = std::env::var("API_KEY").unwrap_or("c4e826b02a1d4771a85163426240404".to_string());
    loop {
        println!("Enter a city name (or 'exit' to quit):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        if input == "exit" {
            println!("Exiting...");
            break;
        }

        let weather_data = fetch_weather(input, &api_key).await;
        match weather_data {
            Ok(data) => {
                print_weather_data(&data);
            }
            Err(e) => {
                eprintln!("Error fetching weather data: {}", e);
            }
        }
    }
}
