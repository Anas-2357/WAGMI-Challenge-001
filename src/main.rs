use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use chrono::Utc;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct AddRequest {
    a: u32,
    b: u32,
}

#[derive(Serialize)]
struct AddResponse {
    a: u32,
    b: u32,
    result: u32,
    status: String,
}

#[tokio::main]
async fn main() {
    // 🔥 Use Railway-provided PORT (defaults to 8080)
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("🚀 Listening on http://{}", listener.local_addr().unwrap());

    let app = Router::new().route("/wagmi", post(wagmi_handler));
    axum::serve(listener, app).await.unwrap();
}

async fn wagmi_handler(Json(payload): Json<Value>) -> Json<Value> {
    let a = payload.get("a").and_then(|v| v.as_u64());
    let b = payload.get("b").and_then(|v| v.as_u64());

    if a.is_some() || b.is_some() {
        if let (Some(a_num), Some(b_num)) = (a, b) {
            if a_num <= 100 && b_num <= 100 && (a_num + b_num) <= 100 {
                let response = AddResponse {
                    a: a_num as u32,
                    b: b_num as u32,
                    result: (a_num + b_num) as u32,
                    status: "success".to_string(),
                };
                return Json(json!(response));
            }
        }
        return Json(json!({ "error": "Invalid input" }));
    }

    Json(json!({
        "message": "wagmi",
        "timestamp": Utc::now().to_rfc3339(),
        "lang": "Rust"
    }))
}
