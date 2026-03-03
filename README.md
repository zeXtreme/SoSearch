# SoSearch API

A lightning-fast pseudo Web Search Engine API written in **Rust** (RIIR - Rewrite It In Rust style).
This project emulates popular APIs like *SerpAPI* or *Tavily* without needing official and expensive API keys, by multiplexing requests to popular engines directly and scraping the results concurrently.

## Philosophy

- **Performance**: Powered by `tokio` for async concurrent I/O.
- **Bot Bypass**: Leverages `rquest` with TLS impersonation (e.g., simulating a Chrome 124 browser footprint at the TLS/HTTP2 layer) to minimize blocking vs standard HTTP clients (the Rust equivalent of `curl_cffi`).
- **Standardized**: Normalizes `DuckDuckGo`, `Yahoo`, and `Brave` HTML results into a standardized `SearchResult` JSON array.

## Core Stack
- [Axum](https://github.com/tokio-rs/axum)
- [Tokio](https://tokio.rs/)
- [rquest](https://github.com/0x676e67/rquest)
- [scraper](https://github.com/causal-agent/scraper)

## 🔍 Supported Search Engines

*   **DuckDuckGo** (Primary standard search)
*   **Yahoo** (Powered by Bing)
*   **Brave Search** (Independent index)

Refer to `QUICK_START.md` for running instructions.
