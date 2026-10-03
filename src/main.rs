use axum::{extract::{State, Path, Query, rejection::JsonRejection}, http::StatusCode, response::{IntoResponse, Response}, routing::get, Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, FromRow};
use std::{env, time::Duration};
use validator::ValidateEmail;
use utoipa_swagger_ui::SwaggerUi;

struct ApiError(StatusCode, String);
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, Json(serde_json::json!({"error": self.1}))).into_response()
    }
}
fn bad(message: &str) -> ApiError { ApiError(StatusCode::BAD_REQUEST, message.into()) }
fn db_error(error: sqlx::Error) -> ApiError {
    if let sqlx::Error::Database(ref e) = error {
        match e.code().as_deref() {
            Some("23505") => return ApiError(StatusCode::CONFLICT, "Email already exists".into()),
            Some("23514") | Some("22003") => return bad("Data violates database constraints"),
            _ => {}
        }
    }
    eprintln!("Database error: {error}");
    ApiError(StatusCode::INTERNAL_SERVER_ERROR, "Database operation failed".into())
}
fn body<T>(input: Result<Json<T>, JsonRejection>) -> Result<T, ApiError> {
    input.map(|Json(value)| value).map_err(|e| ApiError(e.status(), e.body_text()))
}
#[derive(Deserialize)]
struct Pagination { limit: Option<i64>, offset: Option<i64> }
impl Pagination {
    fn values(&self) -> Result<(i64, i64), ApiError> {
        let limit = self.limit.unwrap_or(20);
        let offset = self.offset.unwrap_or(0);
        if !(1..=100).contains(&limit) || offset < 0 { return Err(bad("limit must be 1–100; offset must be nonnegative")); }
        Ok((limit, offset))
    }
}
#[derive(Serialize, FromRow)]
struct User { id: i64, name: String, email: String, phone: Option<String>, created_at: DateTime<Utc>, updated_at: DateTime<Utc> }
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct UserInput { name: String, email: String, phone: Option<String> }
impl UserInput {
    fn clean(mut self) -> Result<Self, ApiError> {
        self.name = self.name.trim().into();
        self.email = self.email.trim().to_lowercase();
        if self.name.is_empty() { return Err(bad("Name is required")); }
        if !self.email.validate_email() { return Err(bad("Valid email is required")); }
        self.phone = self.phone.map(|v| v.trim().to_string()).filter(|v| !v.is_empty());
        Ok(self)
    }
}
#[derive(Serialize, FromRow)]
struct Product { id: i64, name: String, description: Option<String>, price: String, stock: i32, category: Option<String>, created_at: DateTime<Utc> }
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProductInput { name: String, description: Option<String>, price: String, #[serde(default)] stock: i32, category: Option<String> }
impl ProductInput {
    fn clean(mut self) -> Result<Self, ApiError> {
        self.name = self.name.trim().into();
        if self.name.is_empty() || self.stock < 0 { return Err(bad("Name is required and stock must be nonnegative")); }
        let parts: Vec<_> = self.price.split('.').collect();
        let digits = |s: &str| !s.is_empty() && s.bytes().all(|c| c.is_ascii_digit());
        if parts.len() > 2 || !digits(parts[0]) || parts[0].len() > 10 || (parts.len() == 2 && (!digits(parts[1]) || parts[1].len() > 2)) {
            return Err(bad("Price must be a decimal string, e.g. 199.99, with at most 10 integer and 2 decimal digits"));
        }
        Ok(self)
    }
}
async fn health(State(pool): State<PgPool>) -> Result<Json<serde_json::Value>, ApiError> {
    sqlx::query("SELECT 1").execute(&pool).await.map_err(|e| { eprintln!("Health check: {e}"); ApiError(StatusCode::SERVICE_UNAVAILABLE, "Database unavailable".into()) })?;
    Ok(Json(serde_json::json!({"status": "healthy"})))
}

async fn list_users(State(pool): State<PgPool>, Query(page): Query<Pagination>) -> Result<Json<Vec<User>>, ApiError> {
    let (limit, offset) = page.values()?;
    let rows = sqlx::query_as::<_, User>("SELECT id, name, email, phone, created_at, updated_at FROM users ORDER BY id LIMIT $1 OFFSET $2")
        .bind(limit).bind(offset).fetch_all(&pool).await.map_err(db_error)?;
    Ok(Json(rows))
}
async fn get_user(State(pool): State<PgPool>, Path(id): Path<i64>) -> Result<Json<User>, ApiError> {
    let row = sqlx::query_as::<_, User>("SELECT id, name, email, phone, created_at, updated_at FROM users WHERE id=$1")
        .bind(id).fetch_optional(&pool).await.map_err(db_error)?
        .ok_or(ApiError(StatusCode::NOT_FOUND, "User not found".into()))?;
    Ok(Json(row))
}
async fn create_user(State(pool): State<PgPool>, input: Result<Json<UserInput>, JsonRejection>) -> Result<(StatusCode, Json<User>), ApiError> {
    let input = body(input)?.clean()?;
    let row = sqlx::query_as::<_, User>("INSERT INTO users (name, email, phone) VALUES ($1, $2, $3) RETURNING id, name, email, phone, created_at, updated_at")
        .bind(input.name).bind(input.email).bind(input.phone).fetch_one(&pool).await.map_err(db_error)?;
    Ok((StatusCode::CREATED, Json(row)))
}
async fn update_user(State(pool): State<PgPool>, Path(id): Path<i64>, input: Result<Json<UserInput>, JsonRejection>) -> Result<Json<User>, ApiError> {
    let input = body(input)?.clean()?;
    let row = sqlx::query_as::<_, User>("UPDATE users SET name=$1, email=$2, phone=$3, updated_at=NOW() WHERE id=$4 RETURNING id, name, email, phone, created_at, updated_at")
        .bind(input.name).bind(input.email).bind(input.phone).bind(id).fetch_optional(&pool).await.map_err(db_error)?
        .ok_or(ApiError(StatusCode::NOT_FOUND, "User not found".into()))?;
    Ok(Json(row))
}
async fn delete_user(State(pool): State<PgPool>, Path(id): Path<i64>) -> Result<StatusCode, ApiError> {
    let result = sqlx::query("DELETE FROM users WHERE id=$1").bind(id).execute(&pool).await.map_err(db_error)?;
    if result.rows_affected() == 0 { return Err(ApiError(StatusCode::NOT_FOUND, "User not found".into())); }
    Ok(StatusCode::NO_CONTENT)
}

async fn list_products(State(pool): State<PgPool>, Query(page): Query<Pagination>) -> Result<Json<Vec<Product>>, ApiError> {
    let (limit, offset) = page.values()?;
    let rows = sqlx::query_as::<_, Product>("SELECT id, name, description, price::text AS price, stock, category, created_at FROM products ORDER BY id LIMIT $1 OFFSET $2")
        .bind(limit).bind(offset).fetch_all(&pool).await.map_err(db_error)?;
    Ok(Json(rows))
}
async fn get_product(State(pool): State<PgPool>, Path(id): Path<i64>) -> Result<Json<Product>, ApiError> {
    let row = sqlx::query_as::<_, Product>("SELECT id, name, description, price::text AS price, stock, category, created_at FROM products WHERE id=$1")
        .bind(id).fetch_optional(&pool).await.map_err(db_error)?
        .ok_or(ApiError(StatusCode::NOT_FOUND, "Product not found".into()))?;
    Ok(Json(row))
}
async fn create_product(State(pool): State<PgPool>, input: Result<Json<ProductInput>, JsonRejection>) -> Result<(StatusCode, Json<Product>), ApiError> {
    let input = body(input)?.clean()?;
    let row = sqlx::query_as::<_, Product>("INSERT INTO products (name, description, price, stock, category) VALUES ($1, $2, $3::text::numeric, $4, $5) RETURNING id, name, description, price::text AS price, stock, category, created_at")
        .bind(input.name).bind(input.description).bind(input.price).bind(input.stock).bind(input.category).fetch_one(&pool).await.map_err(db_error)?;
    Ok((StatusCode::CREATED, Json(row)))
}
async fn update_product(State(pool): State<PgPool>, Path(id): Path<i64>, input: Result<Json<ProductInput>, JsonRejection>) -> Result<Json<Product>, ApiError> {
    let input = body(input)?.clean()?;
    let row = sqlx::query_as::<_, Product>("UPDATE products SET name=$1, description=$2, price=$3::text::numeric, stock=$4, category=$5 WHERE id=$6 RETURNING id, name, description, price::text AS price, stock, category, created_at")
        .bind(input.name).bind(input.description).bind(input.price).bind(input.stock).bind(input.category).bind(id).fetch_optional(&pool).await.map_err(db_error)?
        .ok_or(ApiError(StatusCode::NOT_FOUND, "Product not found".into()))?;
    Ok(Json(row))
}
async fn delete_product(State(pool): State<PgPool>, Path(id): Path<i64>) -> Result<StatusCode, ApiError> {
    let result = sqlx::query("DELETE FROM products WHERE id=$1").bind(id).execute(&pool).await.map_err(db_error)?;
    if result.rows_affected() == 0 { return Err(ApiError(StatusCode::NOT_FOUND, "Product not found".into())); }
    Ok(StatusCode::NO_CONTENT)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let port: u16 = env::var("PORT").unwrap_or_else(|_| "3002".into()).parse()?;
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port)).await.map_err(|e| {
        eprintln!("Cannot bind port {port}: {e}. Stop the other server or change PORT in .env."); e
    })?;
    let pool = PgPoolOptions::new().max_connections(5).acquire_timeout(Duration::from_secs(10))
        .connect(&env::var("DATABASE_URL")?).await?;
    let spec = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("openapi.json"))?;
    let app = Router::new()
        .route("/", get(|| async { "Rust users and products API" }))
        .route("/health", get(health))
        .route("/users", get(list_users).post(create_user))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .route("/products", get(list_products).post(create_product))
        .route("/products/{id}", get(get_product).put(update_product).delete(delete_product))
        .merge(SwaggerUi::new("/swagger-ui").external_url_unchecked("/api-docs/openapi.json", serde_json::from_str::<serde_json::Value>(&spec)?))
        .with_state(pool);
    println!("API: http://localhost:{port}\nSwagger: http://localhost:{port}/swagger-ui/");
    axum::serve(listener, app).with_graceful_shutdown(async { let _ = tokio::signal::ctrl_c().await; }).await?;
    Ok(())
}
