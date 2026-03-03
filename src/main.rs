mod engines;
mod models;

use axum::{
    extract::Query,
    response::Json,
    routing::get,
    Router,
};
use serde::Deserialize;
use futures::stream::{FuturesUnordered, StreamExt};
use engines::{duckduckgo::DuckDuckGo, brave::Brave, yahoo::Yahoo, SearchEngine};
use models::SearchResponse;
use rquest::Client;
use tracing::info;

#[derive(Deserialize)]
struct SearchParams {
    q: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    // Setup Axum app
    let app = Router::new().route("/search", get(search_handler));

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "10080".to_string())
        .parse()
        .unwrap_or(10080);
        
    println!("API server listening on 0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn search_handler(Query(params): Query<SearchParams>) -> Json<SearchResponse> {
    // Attempt to build a client with Chrome impersonation to avoid basic bot detection
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
        .build()
        .unwrap_or_else(|_| Client::new());

    let rust_engines: Vec<SearchEngine> = vec![
        SearchEngine::DuckDuckGo(DuckDuckGo),
        SearchEngine::Brave(Brave),
        SearchEngine::Yahoo(Yahoo),
    ];

    let mut results = Vec::new();
    let query = params.q.clone();

    // Spawn concurrent tasks for each search engine
    let mut tasks = FuturesUnordered::new();
    for engine in rust_engines {
        let q = query.clone();
        let c = client.clone();
        tasks.push(tokio::spawn(async move {
            let name = engine.name();
            match engine.search(&q, &c).await {
                Ok(items) => {
                    info!("{} returned {} results", name, items.len());
                    items
                }
                Err(e) => {
                    eprintln!("Error searching {}: {}", name, e);
                    vec![]
                }
            }
        }));
    }

    while let Some(res) = tasks.next().await {
        if let Ok(mut items) = res {
            results.append(&mut items);
        }
    }

    Json(SearchResponse {
        query: params.q,
        results: results,
    })
}
