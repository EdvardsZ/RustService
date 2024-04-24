
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

pub fn create_connection_pool() -> Arc<Pool<ConnectionManager<PgConnection>>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");


    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool.");

    Arc::new(pool)
}