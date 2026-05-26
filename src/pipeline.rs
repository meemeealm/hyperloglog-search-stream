use crate::hll::HyperLogLog;
use chrono::{DateTime, Datelike, Utc};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;

#[derive(Debug, Clone, Deserialize)]
pub struct SearchLogEvent {
    pub query: String,
    pub user_id: String,
    pub timestamp: DateTime<Utc>,
}

pub type AnalyticsStore = Arc<RwLock<HashMap<String, HashMap<String, HyperLogLog>>>>;

#[derive(Clone)]
pub struct AppEngineState {
    pub store: AnalyticsStore,
    pub pipeline_tx: mpsc::Sender<SearchLogEvent>,
}

pub fn start_streaming_worker(
    mut pipeline_rx: mpsc::Receiver<SearchLogEvent>,
    store: AnalyticsStore,
) {
    tokio::spawn(async move {
        println!("Data Engine Background Worker running independently...");
        while let Some(event) = pipeline_rx.recv().await {
            let date_bucket = format!(
                "{:04}-{:02}-{:02}",
                event.timestamp.year(),
                event.timestamp.month(),
                event.timestamp.day()
            );

            let mut storage = store.write().unwrap();
            let daily_storage = storage.entry(date_bucket).or_insert_with(HashMap::new);
            let hll = daily_storage
                .entry(event.query)
                .or_insert_with(|| HyperLogLog::new(12));

            hll.insert(&event.user_id);
        }
    });
}
