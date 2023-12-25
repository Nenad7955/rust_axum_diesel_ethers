use axum::{extract::State, Json, Router, routing::get};
use axum::extract::Query;
use dotenv::dotenv;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

use crate::database::db::DB;
use crate::database::user;

mod database;

#[tokio::main]
async fn main() {
    dotenv().ok(); // This line loads the environment variables from the ".env" file.

    let addr = "postgres://db:db@localhost/db".to_string();
    let db = DB::new(addr);

    let conn = db.get_connection().await;
    let app_state = AppState { conn };

    let app = Router::new()
        .route("/", get(list_users).post(create_user).delete(delete_user))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[axum::debug_handler]
async fn list_users(
    state: State<AppState>,
) -> axum::response::Result<Json<Vec<user::Model>>, String> {
    let data = user::Entity::find().all(&state.conn).await.unwrap();

    Ok(Json(data))
}

#[derive(Serialize, Deserialize)]
struct CreateUser {
    pub pub_key: String,
    pub username: String,
}

#[axum::debug_handler]
async fn create_user(
    state: State<AppState>,
    Json(create_user): Json<CreateUser>,
) -> axum::response::Result<Json<user::Model>, String> {
    let data = user::ActiveModel {
        pub_key: Set(create_user.pub_key),
        username: Set(create_user.username),
        ..Default::default()
        // pub_key: create_user.pub_key,
        // username: create_user.username,
    }.insert(&state.conn).await.unwrap();

    Ok(Json(data.into()))
}

#[derive(Deserialize, Debug)]
struct Pagination {
    page: Option<usize>,
    per_page: Option<usize>,
}

#[derive(Deserialize, Debug)]
struct UserId {
    id: i32,
}

#[axum::debug_handler]
async fn delete_user(
    state: State<AppState>,
    user: Query<UserId>,
) -> axum::response::Result<String, String> {
    let user = user.0;

    let res = user::Entity::delete_by_id(user.id).exec(&state.conn).await.unwrap();

    if res.rows_affected > 0 {
        Ok("OK".to_string())
    }
    else {
        Err("Couldn't find by that id".to_string())
    }
}



/*2.0 validator block creator

tx in mem pool coinbase

evm dynamic memory 0x40

private to public key*/
