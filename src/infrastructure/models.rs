use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::calculations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CalculationDto {
    pub id: i32,
    pub temperature: f64,
    pub wind_speed: f64,
    pub relative_humidity: f64,

    pub dew_point: f64,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::calculations)]
pub struct NewCalculationDto {
    pub temperature: f64,
    pub wind_speed: f64,
    pub relative_humidity: f64,

    pub dew_point: f64,
}