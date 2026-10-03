# Rust Users and Products API

Axum + SQLx + PostgreSQL + Swagger UI. This local learning backend has no authentication.

## Run on your Mac

1. Install Rust if needed.
2. Copy `.env.example` to `.env` and set your existing DATABASE_URL and a free PORT.
3. Install the migration CLI: `cargo install sqlx-cli --locked --no-default-features --features rustls,postgres`.
4. For an EMPTY database: run `sqlx migrate run` from this project directory.
5. Run `cargo run`, or install `cargo-watch` and run `cargo watch -x run`.
6. Open http://localhost:3002/swagger-ui/ (use your configured port).

## Existing database from our conversation

Do not run the bundled initial migration if users/products already exist. Do not overwrite your previous migration history. Copy src/main.rs and openapi.json into your existing project and add the dependencies from Cargo.toml. Keep your existing .env and migrations.

Expected users columns: id BIGINT, name TEXT, email TEXT UNIQUE, phone TEXT nullable, created_at/updated_at TIMESTAMPTZ.
Expected products columns: id BIGINT, name TEXT, description TEXT nullable, price NUMERIC(12,2), stock INTEGER, category TEXT nullable, created_at TIMESTAMPTZ.
If category is missing, create a new migration with `sqlx migrate add add_product_category`, put `ALTER TABLE products ADD COLUMN category TEXT;` in it, then run `sqlx migrate run`. Add any other missing columns through new migrations, inspecting the existing database first.

Migrations run explicitly before starting the application; they are not run at startup. Keep applied SQL files unchanged. Future schema changes belong in new migrations.

## Routes

GET /health: database readiness.
GET/POST /users and /products: list/create.
GET/PUT/DELETE /users/{id} and /products/{id}: fetch/replace/delete.
Lists accept limit (1–100, default 20) and offset (>=0, default 0).
POST returns 201, DELETE returns 204, missing records return 404, duplicate user email returns 409.
PUT is replacement: send name/email for users and name/price for products. Omitted optional values become null and omitted stock becomes zero.
Prices are JSON strings such as "199.99" to avoid floating point rounding. Maximum is "9999999999.99". No user/product ownership relationship is assumed.
User email is trimmed/lowercased; API email uniqueness uses that normalized value. Previously inserted mixed-case emails should be normalized after checking for collisions.
Malformed path/query inputs may use Axum's plain text error response; application errors and JSON body rejections return {"error":"..."}.

## Swagger

Expand POST /users or POST /products, click Try it out, edit JSON, Execute. Update openapi.json whenever changing API behavior. Restart after .env changes. The API never exposes the database URL.

## Validation performed

OpenAPI JSON structure and route coverage checked. Rust compilation and live PostgreSQL integration must be checked on your machine because this generation environment has no Rust compiler or PostgreSQL. Run `cargo check` then test create/list/get/update/delete through Swagger.
