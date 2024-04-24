pub mod application;
pub mod infrastructure;
pub mod domain;
pub mod schema;

use std::time::Duration;
use application::app_service::AppService;
use tokio::{time, task};
use std::sync::Arc;



async fn fetch_data_and_calculate(shared_service: Arc<AppService>) {
    loop {
        shared_service.fetch_and_calculate().await;

        time::sleep(Duration::from_secs(1)).await; // Fetch and calculate every second
    }
}

async fn store_data_periodically(shared_service: Arc<AppService>) {
    loop {
        shared_service.store_data().await;

        time::sleep(Duration::from_secs(10)).await; // Store every 10 seconds
    }
}

#[tokio::main]
async fn main() {

    let shared_app_service = Arc::new(AppService::new());
    // Spawn tasks for data retrieval, metrics calculation, and data storage
    let fetch_calculate_task = task::spawn(fetch_data_and_calculate(shared_app_service.clone()));
    let store_task = task::spawn(store_data_periodically(shared_app_service.clone()));

    // Wait for all tasks to finish
    fetch_calculate_task.await.unwrap();
    store_task.await.unwrap();
}