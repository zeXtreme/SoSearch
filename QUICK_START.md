# Quick Start

Follow these steps to build and run the `SoSearch` API layer locally. 
This project has been set up with a China-based rust mirror (`rsproxy` inside `.cargo/config.toml`) to ensure fast and reliable builds.

## 1. Prerequisites
- [Rust & Cargo](https://rustup.rs/) configured on your environment.
- Ports `8080` available locally.

## 2. Running Locally

Start the Axum server:

```bash
cargo run --release
```

To run it in the background:
```bash
cargo run --release > server.log 2>&1 &
```

## 3. Usage Example

Ensure your proxy or local connection is stable, then request a search using `curl`:

```bash
curl -s "http://localhost:8080/search?q=rust+lang" | jq .
```

### Example Output

```json
{
  "query": "rust lang",
  "results": [
    {
      "title": "Rust Programming Language",
      "url": "https://rust-lang.org/",
      "snippet": "Rust is a fast, reliable, and productive programming language...",
      "engine": "duckduckgo"
    }
  ]
}
```

> **Note**: For heavy scraping requirements against Google and Yandex, consider proxy usage or running a Headless browser backend. The `rquest` crate's TLS impersonation takes you quite far but can still be flagged after bulk usage from datacenter IPs.
