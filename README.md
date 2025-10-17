# 🐱 Dynamic Profile API (Rust + Axum)

Production‑ready API for **HNG Backend Stage 0**.

It returns:

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
- 🌐 Reqwest HTTP client with per‑request timeout  
- 🧰 Clean error mapping (upstream → simple error JSON + correct status code)  
- 📄 OpenAPI (Utoipa) + Swagger UI at `/docs`  
- 📊 Prometheus‑compatible metrics at `/metrics`  
- 🧪 Integration tests  
- 🔁 Circuit‑breaker friendly architecture (pluggable)  
- 🔭 Observability (tracing logs + request latency metrics)

---

## 🧰 Tech Stack

| Layer       | Tool                          |
|------------|--------------------------------|
| Language    | Rust                          |
| Web Server  | Axum                          |
| Async       | Tokio                         |
| HTTP Client | Reqwest                       |
| Docs        | Utoipa + Swagger UI           |
| Metrics     | Prometheus (text exposition)  |
| Logs        | `tracing` + `tracing-subscriber` |

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

### ❌ Failure (HTTP error mapped from upstream)

```json
{
  "status": "failed",
  "message": "Cat oracle is taking too long ⏳",
  "status_code": 504
}
```

#### Failure & Error Mapping

| Failure class                      | Example cause                           | HTTP status | Client payload (example)                                              |
|-----------------------------------|-----------------------------------------|-------------|------------------------------------------------------------------------|
| Timeout                           | Upstream took too long                  | 504         | `{"status":"failed","message":"Cat oracle is taking too long ⏳","status_code":504}` |
| Upstream 4xx                      | 400/404/429 from cat API                | 502         | `{"status":"failed","message":"Upstream rejected the request","status_code":502}`   |
| Upstream 5xx                      | 500/502/503 from cat API                | 503         | `{"status":"failed","message":"Upstream service unavailable","status_code":503}`    |
| Network / DNS / TLS               | Connection refused, name not resolved   | 502         | `{"status":"failed","message":"Network error reaching upstream","status_code":502}` |
| Invalid/Unexpected JSON structure | Upstream changed response schema        | 502         | `{"status":"failed","message":"Bad response from upstream","status_code":502}`      |

> The server logs capture full diagnostics; the client receives a safe, normalized error.

---

## 🗺️ Endpoints

- `GET /me` — Returns profile info, current UTC timestamp, and a fresh cat fact  
  - **200 OK** → Success payload (see above)  
  - **5xx/502/503/504** → Failure payload (normalized)  
- `GET /metrics` — Prometheus metrics (text/plain; scrape‑ready)  
- `GET /docs` — Swagger UI (OpenAPI available at `/api-doc/openapi.json`)

### Quick cURL

```bash
# Profile
curl -i http://localhost:8080/me

# Metrics
curl -i http://localhost:8080/metrics

# OpenAPI JSON
curl -i http://localhost:8080/api-doc/openapi.json
```

---

## 🗂️ Project Structure

```
src/
├── main.rs                 # Entrypoint (bind server, logging, layers)
├── config/
│   └── mod.rs              # AppConfig (PORT, timeouts)
├── routes/
│   └── mod.rs              # create_router (routes + docs mounting)
├── models.rs               # ProfileResponse, ErrorResponse, User
├── handlers/
│   └── profile.rs          # GET /me (success/error mapping)
├── services/
│   └── cat_service.rs      # Cat fact + typed errors (timeout/4xx/5xx/network/json)
└── metrics/
    └── mod.rs              # Prometheus registry, /metrics, middleware
```

---

## ⚙️ Configuration

Create `.env` (copy from example):

```env
# .env
PORT=8080
RUST_LOG=info,axum=info,tower_http=info
```

Inline overrides:

```bash
# macOS/Linux
PORT=9090 RUST_LOG=debug cargo run

# PowerShell
$env:PORT=9090; $env:RUST_LOG="debug"; cargo run
```

---

## 🧪 Run Locally

```bash
# 1) Install Rust
#    https://rustup.rs

# 2) Prepare env
cp .env.example .env

# 3) Run with logs
RUST_LOG=info,axum=info,tower_http=info cargo run
# => 🚀 Listening on http://0.0.0.0:8080

# 4) Hit the API
curl -i http://localhost:8080/me

# 5) Open Swagger UI
# macOS:
open http://localhost:8080/docs
# Linux:
xdg-open http://localhost:8080/docs

# 6) Prometheus metrics
curl -i http://localhost:8080/metrics
```

---

## 🧱 Build & Test

```bash
# Build release binary
cargo build --release

# Run tests (unit + integration where provided)
cargo test
```

---

## 🐳 Docker (Optional)

```bash
# Build image
docker build -t dynamic-profile-api:latest .

# Run container
docker run --rm -p 8080:8080 -e PORT=8080 dynamic-profile-api:latest

# Verify
curl -i http://localhost:8080/me
```

---

## 🔭 Observability

- **Logging:** Structured JSON logs via `tracing` (request IDs, method, path, status, latency).
- **Metrics:** Request counts, latencies, and error rates exposed at `/metrics`.
- **Tracing:** Middleware attaches spans to each request for end‑to‑end timing.

---

## 🛡️ Error Handling (Design Summary)

- All upstream errors are normalized into `ErrorResponse` with:
  - `status: "failed"`
  - Human‑readable `message`
  - Correct `status_code` to match the failure category
- Handlers remain thin; services classify errors so mapping is consistent and testable.

---

## 🧩 Implementation Notes

- **Timeouts:** Each upstream call (cat fact) uses a per‑request timeout to avoid tail latencies.
- **Resiliency:** The service layer is designed so you can easily add a circuit breaker/retry policy later.
- **Separation:** Handlers are thin; services contain network logic + error typing; models are shared.

---

## 📦 Deployment

- Provide your host URL in **Live API** section once deployed.
- Works on most platforms (Fly.io, Railway, Render, Hetzner, DigitalOcean, AWS). Exposes `PORT` from env.

---

## 📄 License

MIT (or your preferred license)