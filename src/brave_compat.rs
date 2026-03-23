use axum::{
    extract::Query,
    response::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::search::perform_search;

#[derive(Deserialize)]
pub struct BraveSearchParams {
    q: String,
    count: Option<usize>,
}

#[derive(Serialize)]
pub struct BraveWebResult {
    pub title: String,
    pub url: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct BraveWebResults {
    pub results: Vec<BraveWebResult>,
}

#[derive(Serialize)]
pub struct BraveQueryInfo {
    pub original: String,
}

#[derive(Serialize)]
pub struct BraveSearchResponse {
    pub query: BraveQueryInfo,
    pub web: BraveWebResults,
}

pub async fn brave_search_handler(
    Query(params): Query<BraveSearchParams>,
) -> Result<Json<BraveSearchResponse>, StatusCode> {
    let mut results = perform_search(&params.q).await;

    // Deduplicate by URL, keep first occurrence
    let mut seen = std::collections::HashSet::new();
    results.retain(|r| seen.insert(r.url.clone()));

    // Limit to requested count (default 10)
    let count = params.count.unwrap_or(10);
    results.truncate(count);

    let web_results: Vec<BraveWebResult> = results
        .into_iter()
        .map(|r| BraveWebResult {
            title: r.title,
            url: r.url,
            description: r.snippet,
        })
        .collect();

    Ok(Json(BraveSearchResponse {
        query: BraveQueryInfo {
            original: params.q,
        },
        web: BraveWebResults {
            results: web_results,
        },
    }))
}
