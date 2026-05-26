use chrono::Datelike;
use std::sync::{Arc, RwLock};

mod hll;
mod pipeline;

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

use pipeline::{start_streaming_worker, AppEngineState, SearchLogEvent};

#[derive(Deserialize)]
struct IncomingPayload {
    query: String,
    user_id: String,
}

#[derive(Deserialize)]
struct DateParam {
    date: Option<String>,
}

#[derive(Serialize)]
struct AnalyticsResponse {
    date: String,
    query: String,
    estimated_unique_users: u64,
}

async fn ingest_search_event(
    State(engine): State<AppEngineState>,
    Json(payload): Json<IncomingPayload>,
) -> Result<Json<String>, (axum::http::StatusCode, String)> {
    let event = SearchLogEvent {
        query: payload.query.to_lowercase().trim().to_string(),
        user_id: payload.user_id,
        timestamp: Utc::now(),
    };

    match engine.pipeline_tx.send(event).await {
        Ok(_) => Ok(Json("Event queued in streaming pipeline".to_string())),
        Err(_) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Ingestion pipeline buffer full or disconnected".to_string(),
        )),
    }
}

async fn read_analytics(
    Path(query): Path<String>,
    Query(params): Query<DateParam>,
    State(engine): State<AppEngineState>,
) -> Json<AnalyticsResponse> {
    let date_bucket = params.date.unwrap_or_else(|| {
        let now = Utc::now();
        format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day())
    });

    let query_clean = query.to_lowercase().trim().to_string();
    let storage = engine.store.read().unwrap();

    let estimated_unique_users = storage
        .get(&date_bucket)
        .and_then(|daily| daily.get(&query_clean))
        .map(|hll| hll.estimate() as u64)
        .unwrap_or(0);

    Json(AnalyticsResponse {
        date: date_bucket,
        query: query_clean,
        estimated_unique_users,
    })
}

#[tokio::main]
async fn main() {
    let analytical_store = Arc::new(RwLock::new(HashMap::new()));
    let (tx, rx) = mpsc::channel::<SearchLogEvent>(10_000);

    start_streaming_worker(rx, Arc::clone(&analytical_store));

    let engine_state = AppEngineState {
        store: analytical_store,
        pipeline_tx: tx,
    };

    let app = Router::new()
        .route("/track", post(ingest_search_event))
        .route("/analytics/:query", get(read_analytics))
        .with_state(engine_state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Decoupled Production Data Engine running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
