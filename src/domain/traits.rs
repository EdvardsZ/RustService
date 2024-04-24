use crate::domain::{error::DomainError, models::RealTimeData};
use async_trait::async_trait;

#[async_trait]
pub trait DataService {
    async fn retrieve_real_time_data(&self) -> Result<RealTimeData, DomainError>;
}