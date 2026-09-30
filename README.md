# Axum Notes API

A RESTful Notes API built with Rust, Axum, SQLx, and PostgreSQL.

This project was built to learn and demonstrate backend development in Rust, including REST API design, database integration, Docker-based development, and asynchronous programming.

## Tech Stack

* Rust
* Axum
* SQLx
* PostgreSQL
* Docker & Docker Compose
* Serde
* Tokio

## Features

* Health check endpoint
* Create notes
* List notes with pagination
* Retrieve a note by ID
* Update notes
* Delete notes
* PostgreSQL persistence
* UUID-based note IDs
* Dockerized PostgreSQL and pgAdmin setup

## Project Structure

```text
axum-notes-api/
├── migrations/
│   └── 001_create_notes.sql
├── src/
│   ├── handler.rs
│   ├── main.rs
│   ├── model.rs
│   ├── route.rs
│   └── schema.rs
├── .env.example
├── .gitignore
├── Cargo.lock
├── Cargo.toml
└── docker-compose.yml
```

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/joaorodrigues0764/axum-notes-api.git
cd axum-notes-api
```

### 2. Create the environment file

Copy `.env.example` to `.env`:

```bash
cp .env.example .env
```

Update the values in `.env` as needed.

The `.env` file is intentionally ignored by Git and must not be committed.

### 3. Start PostgreSQL and pgAdmin

```bash
docker compose up -d
```

Check that the containers are running:

```bash
docker ps
```

### 4. Create the database table

For a fresh database, apply the migration:

```bash
docker exec -i postgres psql -U admin -d rust_sqlx < migrations/001_create_notes.sql
```

### 5. Run the API

```bash
cargo run
```

The API will start on:

```text
http://127.0.0.1:8000
```

## API Endpoints

### Health Check

```http
GET /api/healthchecker
```

### List Notes

```http
GET /api/notes
```

Optional pagination parameters:

```text
?page=1&limit=10
```

### Create Note

```http
POST /api/notes/
Content-Type: application/json
```

Example request:

```json
{
  "title": "My first note",
  "content": "Learning Rust and Axum",
  "category": "programming"
}
```

### Get Note

```http
GET /api/notes/:id
```

### Update Note

```http
PATCH /api/notes/:id
Content-Type: application/json
```

Example request:

```json
{
  "title": "Updated note",
  "content": "Learning Rust, Axum, SQLx and PostgreSQL",
  "category": "rust",
  "published": true
}
```

### Delete Note

```http
DELETE /api/notes/:id
```

## Example

Create a note:

```bash
curl -X POST http://127.0.0.1:8000/api/notes/ \
  -H "Content-Type: application/json" \
  -d '{
    "title": "My first note",
    "content": "Learning Rust and Axum",
    "category": "programming"
  }'
```

Then retrieve all notes:

```bash
curl http://127.0.0.1:8000/api/notes
```

## Database

PostgreSQL runs inside Docker and is exposed locally on port `6500`.

pgAdmin is available locally on port `5050`.

Database configuration is provided through environment variables in `.env`.

## Development

Format the code:

```bash
cargo fmt
```

Check the project:

```bash
cargo check
```

Run tests:

```bash
cargo test
```

## License

This project is for educational and portfolio purposes.
