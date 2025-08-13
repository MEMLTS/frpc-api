use axum::{routing, Json, Router, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

mod edit;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", routing::get(|| async { "Hello, World!" }))
        .route("/config/get_value", routing::post(get_config_value))
        .route("/config/set_value", routing::post(set_config_value));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

#[derive(Deserialize,Debug)]
struct GetValue {
    key: String,
}

#[derive(Serialize,Debug)]
struct GetValueRes {
    code: i32,
    key: String,
    value: String,
}

async fn get_config_value(Json(payload): Json<GetValue>) -> impl IntoResponse {
    match edit::get_value("frpc.toml", &payload.key) {
        Ok(val) => Json(GetValueRes {
            code: 0,
            key: payload.key,
            value: val
        }),
        Err(err) => {
            Json(GetValueRes {
                code: -1,
                key: payload.key,
                value: format!("Error: {}", err)
            })
        }
    }
}

#[derive(Deserialize,Debug)]
struct SetValue {
    key: String,
    value: String,
}
#[derive(Serialize,Debug)]
struct SetValueRes {
    code: i32,
}
async fn set_config_value(Json(payload): Json<SetValue>) -> impl IntoResponse {
    match edit::set_value("frpc.toml", &payload.key, &payload.value) {
        Ok(_) => Json(SetValueRes {
            code: 0
        }),
        Err(err) => {
            println!("Error: {}", err);
            Json(SetValueRes {
                code: -1
            })
        }
    }
}