use crate::domain::models::{Calculation, RealTimeData};

pub struct SimpleMetricsCalculator {}

impl Default for SimpleMetricsCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleMetricsCalculator {

    pub fn new() -> Self {
        SimpleMetricsCalculator {}
    }

    pub fn calculate_metrics(&self, data: &RealTimeData) -> Calculation {
        let dew_point = data.temperature - ((100.0 - data.relative_humidity) / 5.0);

        Calculation {
            temperature: data.temperature,
            wind_speed: data.wind_speed,
            relative_humidity: data.relative_humidity,
            dew_point,
        }
    }
}
