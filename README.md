# 🐱 Dynamic Profile API (Rust + Axum)

Production-ready API for **HNG Backend Stage 0**. It returns:

- ✅ Your profile data (email, name, stack)
- 🕒 Current UTC timestamp (ISO 8601)
- 🐈 A dynamic cat fact from `https://catfact.ninja/fact`
- 🛡️ Clean error handling for upstream failures (timeout, 4xx/5xx, network)
- 📄 Swagger docs at `/docs`
- 📊 Prometheus metrics at `/metrics`
- 🔭 Observability via structured logs + request tracing

---

## 🔗 Live API

> Replace with your deployed URL  
**Example:** `https://<your-deployment-url>/me`

---

## 🚀 Features

- 🦀 Rust 2021 + Tokio async runtime
- ⚡ Axum 0.7 web framework
- 🌐 Reqwest HTTP client with per-request timeout
- 🧰 Clean error mapping (upstream → your simple error JSON + correct status code)
- 📄 OpenAPI (Utoipa) + Swagger UI at `/docs`
- 📊 Prometheus-compatible metrics at `/metrics`
- 🧪 Integration tests
- 🔁 Optional circuit-breaker friendly architecture
- 🔭 Observability (tracing logs + request latency metrics)

---

## 🧰 Tech Stack

| Layer       | Tool                          |
|------------|-------------------------------|
| Language    | Rust                          |
| Web Server  | Axum                          |
| Async       | Tokio                         |
| HTTP Client | Reqwest                       |
| Docs        | Utoipa + Swagger UI           |
| Metrics     | Prometheus (text exposition)  |
| Logs        | tracing + tracing-subscriber  |

---

## 📚 API Contract

### ✅ Success (HTTP 200)
```json
{
  "status": "success",
  "user": {
    "email": "you@example.com",
    "name": "Your Name",
    "stack": "Rust/Axum"
  },
  "timestamp": "2025-10-15T12:34:56.789Z",
  "fact": "Cats have five toes on their front paws..."
}
```
### ❌ Failure (Upstream error → mapped HTTP code)
```json
{
  "status": "failed",
  "message": "Cat oracle is taking too long ⏳",
  "status_code": 504
}

---

## Project Structure
src/
├── main.rs                   # entrypoint (bind, logging, layers)
├── config/                   # AppConfig (PORT)
│   └── mod.rs
├── routes/                   # create_router (routes + docs)
│   └── mod.rs
├── models.rs                 # ProfileResponse, ErrorResponse, User
├── handlers/
│   └── profile.rs            # GET /me (success or error)
├── services/
│   └── cat_service.rs        # cat fact + typed errors (timeout/4xx/5xx/network/json)
└── metrics/
    └── mod.rs                # Prometheus registry, /metrics, middleware

---

Configuration

Create .env (copy from example):
# .env
PORT=8080
RUST_LOG=info,axum=info,tower_http=info

macOS/Linux inline override:
PORT=9090 RUST_LOG=debug cargo run

PowerShell:
$env:PORT=9090; $env:RUST_LOG="debug"; cargo run


Run Locally
# 1) Install Rust: https://rustup.rs
# 2) Prepare env
cp .env.example .env

# 3) Run with logs
RUST_LOG=info,axum=info,tower_http=info cargo run
# => 🚀 Listening on http://0.0.0.0:8080


Test endpoints:
curl -i http://localhost:8080/me
open  http://localhost:8080/docs
open  http://localhost:8080/metrics

