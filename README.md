# Axum Notes API

A small RESTful Notes API built with Rust, Axum, SQLx, and PostgreSQL.

The project demonstrates backend fundamentals including REST API design, asynchronous Rust, PostgreSQL persistence, database migrations, Docker-based development, input validation, pagination, structured error handling, and basic automated tests.

## Tech Stack

- Rust 2024
- Axum
- SQLx
- PostgreSQL
- Tokio
- Docker & Docker Compose
- Serde / serde_json
- Tracing

## Features

- Health check endpoint
- Create, read, update, and delete notes
- Pagination with validated `page` and `limit` parameters
- UUID-based note IDs
- Optional categories
- Published/unpublished notes
- Input validation
- Centralized API error responses
- Automatic SQLx migrations on application startup
- PostgreSQL and pgAdmin through Docker Compose
- Request tracing
- Basic router tests

## Project Structure

```text
axum-notes-api/
├── migrations/
│   └── 001_create_notes.sql
├── src/
│   ├── error.rs
│   ├── handler.rs
│   ├── main.rs
│   ├── model.rs
│   ├── route.rs
│   └── schema.rs
├── .env.example
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── docker-compose.yml
└── README.md
```

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/joaorodrigues0764/axum-notes-api.git
cd axum-notes-api
```

### 2. Configure environment variables

```bash
cp .env.example .env
```

Edit `.env` with local values. Do not commit `.env`.

### 3. Start PostgreSQL and pgAdmin

```bash
docker compose up -d
```

Check the services:

```bash
docker compose ps
```

### 4. Run the API

```bash
cargo run
```

Database migrations are applied automatically when the application starts.

The API is available at:

```text
http://127.0.0.1:8000
```

pgAdmin is available at:

```text
http://127.0.0.1:5050
```

## API Endpoints

| Method | Endpoint | Description |
|---|---|---|
| GET | `/api/healthchecker` | Health check |
| GET | `/api/notes` | List notes |
| POST | `/api/notes` | Create a note |
| GET | `/api/notes/:id` | Get a note |
| PATCH | `/api/notes/:id` | Update a note |
| DELETE | `/api/notes/:id` | Delete a note |

> On Axum 0.8, path parameters use the `{id}` syntax internally in the router. The HTTP endpoint remains `/api/notes/<uuid>`.

### List notes with pagination

```bash
curl "http://127.0.0.1:8000/api/notes?page=1&limit=10"
```

`limit` must be between 1 and 100.

### Create a note

```bash
curl -X POST http://127.0.0.1:8000/api/notes \
  -H "Content-Type: application/json" \
  -d '{
    "title": "My first note",
    "content": "Learning Rust and Axum",
    "category": "programming",
    "published": false
  }'
```

### Update a note

```bash
curl -X PATCH http://127.0.0.1:8000/api/notes/<UUID> \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Updated note",
    "published": true
  }'
```

### Delete a note

```bash
curl -X DELETE http://127.0.0.1:8000/api/notes/<UUID>
```

## Development

Format the code:

```bash
cargo fmt
```

Check the project:

```bash
cargo check
```

Run Clippy:

```bash
cargo clippy --all-targets --all-features
```

Run tests:

```bash
cargo test
```

## Database

PostgreSQL is exposed locally on port `6500` and pgAdmin on port `5050`.

SQLx embeds the migration files into the binary and applies pending migrations at startup.

## Project Status

Educational and portfolio project focused on learning backend development with Rust.
