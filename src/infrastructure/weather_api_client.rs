use crate::domain::{error::DomainError, models::RealTimeData, traits::DataService};
use reqwest::Client;
use serde::Deserialize;
use async_trait::async_trait;
use dotenvy::dotenv;
use std::env;

pub struct WeatherApiClient {
    client: Client,
    api_url: String,
}

#[derive(Deserialize)]
struct WeatherDataResponse
{
    //latitude: f64,
    //longitude: f64,
    current: CurrentWeatherResponse,
}

#[derive(Deserialize)]
struct CurrentWeatherResponse
{
    //interval: f64,
    temperature_2m: f64,
    wind_speed_10m: f64,
    relative_humidity_2m: f64,
}

impl Default for WeatherApiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl WeatherApiClient {
    pub fn new() -> WeatherApiClient {
        let api_url = get_weather_api_url();
        WeatherApiClient {
            client: Client::new(),
            api_url,
        }
    }

    async fn get_weather_data(&self) -> Result<WeatherDataResponse, DomainError> {
        match self.client.get(&self.api_url).send().await {
            Ok(response) => {
                match response.json::<WeatherDataResponse>().await {
                    Ok(weather_data) => Ok(weather_data),
                    Err(_) => Err(DomainError::ApiError("Failed to parse weather data".to_string())),
                }
            },
            Err(_) => Err(DomainError::ApiError("Failed to fetch weather data".to_string())),
        }
    }
}

fn get_weather_api_url() -> String {
    dotenv().ok();
    
    env::var("WEATHER_API_URL").expect("WEATHER_API_URL must be set.")
}

#[async_trait]
impl DataService for WeatherApiClient {
    async fn retrieve_real_time_data(&self) -> Result<RealTimeData, DomainError> {
        match self.get_weather_data().await {
            Ok(weather_data) => {
                Ok(RealTimeData {
                    temperature: weather_data.current.temperature_2m,
                    wind_speed: weather_data.current.wind_speed_10m,
                    relative_humidity: weather_data.current.relative_humidity_2m,
                })
            },
            Err(e) => Err(e),
        }
           
    }
}


