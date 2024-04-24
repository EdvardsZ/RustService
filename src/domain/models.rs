//implement debug
#[derive(Debug)]
pub struct RealTimeData {
    // Define the structure of real-time data
    pub temperature: f64,
    pub wind_speed: f64,
    pub relative_humidity: f64,
}

#[derive(Debug, Clone)]
pub struct Calculation {
    pub temperature: f64,
    pub wind_speed: f64,
    pub relative_humidity: f64,
    pub dew_point: f64,
}

