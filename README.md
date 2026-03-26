# 🎓 Offline Classroom Quiz API

A robust, offline-first backend API built with Rust, Axum, and SQLite. Designed to serve progressive web apps (PWAs) in low-connectivity classroom environments via a local Wi-Fi Hotspot.

## 🚀 Features

* **Zero-Config Database:** Uses embedded SQLite. No database servers to install.
* **Offline Sync:** Idempotent `POST` endpoints to safely synchronize scores when student devices reconnect to the teacher's hotspot.
* **CORS Enabled:** Seamlessly accepts incoming payload data from PWAs regardless of local IP changes.
* **Teacher Dashboard:** Native, zero-dependency SVG QR Code generation for easy student onboarding.
* **Swagger UI:** Auto-generated interactive API documentation.

## 🛠️ Tech Stack

* **Language:** Rust
* **Web Framework:** Axum + Tokio
* **Database:** SQLite (via `sqlx`)
* **Documentation:** Swagger (via `utoipa`)

## 📦 How to Run

1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone this repository.
3. Run the server:

```bash
# Run with an empty database
cargo run

# OR: Run and seed the database with mock data (Recommended for testing)
cargo run -- --seed
🌐 Endpoints & Dashboards

Once the server is running, you can access the following local URLs:

Teacher QR Code Dashboard:http://localhost:3000/admin

Interactive API Docs (Swagger): http://localhost:3000/swagger-ui

📡 Database Architecture (V4)

Independent Entities: Teachers, Classes, Subjects

Dependent Entities: Students, Sessions, Questions, Options

Transactional Entities: Scores (Includes local_attempt_id for idempotent offline synchronization)
