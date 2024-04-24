use std::sync::Mutex;
use colored::*;

use crate::domain::{error::DomainError, models::{Calculation, RealTimeData}, simple_metrics_calculator::SimpleMetricsCalculator, traits::DataService};
use crate::infrastructure::{WeatherApiClient, CalculationRepository};


pub struct AppService {
    pub calculations: Mutex<Vec<Calculation>>,

    pub api_client: WeatherApiClient,
    pub metrics_calculator: SimpleMetricsCalculator,
    pub repository: CalculationRepository,
    

}

impl Default for AppService {
    fn default() -> Self {
        Self::new()
    }
}

impl AppService {
    pub fn new() -> Self {
        AppService {
            calculations: Mutex::new(Vec::new()),

            api_client: WeatherApiClient::new(),
            repository: CalculationRepository::new(),
            metrics_calculator: SimpleMetricsCalculator::new(),
        }
    }

    pub async fn fetch_and_calculate(&self) {
        // Fetch data
        println!("Fetching data from the API and doing calculations...");

        let data = match self.fetch_data().await {
            Ok(data) => data,
            Err(e) => {
                println!("Error fetching data: {:?}", e);
                return; // Return early if there's an error
            },
        };

        println!("\tFetched data from API: {:?}", data);
        // Calculate metrics
        self.calculate_metrics(&data);
    }

    async fn fetch_data(&self) -> Result<RealTimeData, DomainError> {
        // Fetch data from the data servic e
        match self.api_client.retrieve_real_time_data().await {
            Ok(data) => {
                Ok(data)
            },
            Err(e) => {
                println!("Error fetching data: {:?}", e);
                Err(e)
            },
        }
    }

    fn calculate_metrics(&self, data: &RealTimeData) {
        // Calculate metrics
        let metrics = self.metrics_calculator.calculate_metrics(data);
        
        println!("\tMetrics calculated: {:?}", metrics);

        self.calculations.lock().unwrap().push(metrics);
    }

    pub async fn store_data(&self) {
        // Store data in the database
        let yellow_text = "Storing data in the database...".yellow();
        println!("{}",yellow_text);

        
        let calculations = self.calculations.lock().unwrap().clone();

        match self.repository.store_calculations(calculations).await {
            Ok(res) => {
                let length = res.len();
                let green_text = format!("\tData stored successfully: {} calculations stored", length).green();
                println!("{}", green_text);
                self.calculations.lock().unwrap().clear();
            },
            Err(e) => {
                println!("Error storing data: {:?}", e);
            },
        }
        
    }
}