use crate::domain::{error::DomainError, models::Calculation};
use crate::infrastructure::db::create_connection_pool;
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use crate::infrastructure::models::CalculationDto;
use crate::infrastructure::models::NewCalculationDto;
use std::sync::Arc;
use crate::schema::calculations;

pub struct CalculationRepository{
    connection_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl Default for CalculationRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl CalculationRepository{
    pub fn new() -> CalculationRepository{
        CalculationRepository{
            connection_pool: create_connection_pool(),
        }
    }

    pub async fn store_calculations(&self, calculations: Vec<Calculation>) -> Result<Vec<Calculation>, DomainError> {
        let conn = &mut self.connection_pool.get().unwrap();

        let new_calculation_dto: Vec<NewCalculationDto> = calculations.iter().map(|calculation| NewCalculationDto{
            temperature: calculation.temperature,
            wind_speed: calculation.wind_speed,
            relative_humidity: calculation.relative_humidity,
            dew_point: calculation.dew_point,
        }).collect();

        let dtos = diesel::insert_into(calculations::table)
            .values(&new_calculation_dto)
            .returning(CalculationDto::as_returning())
            .load::<CalculationDto>(conn);

        match dtos{
            Ok(dtos) => {
                let calculations: Vec<Calculation> = dtos.iter().map(|dto| Calculation{
                    temperature: dto.temperature,
                    wind_speed: dto.wind_speed,
                    relative_humidity: dto.relative_humidity,
                    dew_point: dto.dew_point,
                }).collect();
                Ok(calculations)
            },
            Err(e) => {
                Err(DomainError::DatabaseError((format!("Failed to store calculations: {}", e)).to_string()))
            }
        }
    }
}


