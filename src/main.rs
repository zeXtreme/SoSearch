mod engines;
mod models;
mod search;
mod mcp;
mod brave_compat;

use axum::{
    extract::Query,
    response::Json,
    routing::get,
    Router,
};
use serde::Deserialize;
use models::SearchResponse;
use search::perform_search;
use brave_compat::brave_search_handler;

#[derive(Deserialize)]
struct SearchParams {
    q: String,
}

#[tokio::main]
async fn main() {
    // Check for --mcp flag
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--mcp") {
        mcp::run().await;
        return;
    }

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/search", get(search_handler))
        .route("/brave/res/v1/web/search", get(brave_search_handler));

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "10080".to_string())
        .parse()
        .unwrap_or(10080);

    println!("API server listening on 0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn search_handler(Query(params): Query<SearchParams>) -> Json<SearchResponse> {
    let results = perform_search(&params.q).await;
    Json(SearchResponse {
        query: params.q,
        results,
    })
}
