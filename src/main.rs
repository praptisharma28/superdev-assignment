use warp::Filter;
use std::collections::HashMap;

mod handlers;
mod types;
mod crypto;
mod solana_ops;

use handlers::*;
use types::*;

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["POST", "GET", "OPTIONS"]);

    let health = warp::path("health")
        .and(warp::get())
        .map(|| {
            warp::reply::json(&ApiResponse::success(serde_json::json!({
                "status": "healthy",
                "service": "solana-http-server"
            })))
        });

    let keypair = warp::path("keypair")
        .and(warp::post())
        .and_then(handle_generate_keypair);

    let create_token = warp::path!("token" / "create")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_create_token);

    let mint_token = warp::path!("token" / "mint")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_mint_token);

    let sign_message = warp::path!("message" / "sign")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_sign_message);

    let verify_message = warp::path!("message" / "verify")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_verify_message);

    let send_sol = warp::path!("send" / "sol")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_send_sol);

    let send_token = warp::path!("send" / "token")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_send_token);

    let routes = health
        .or(keypair)
        .or(create_token)
        .or(mint_token)
        .or(sign_message)
        .or(verify_message)
        .or(send_sol)
        .or(send_token)
        .with(cors)
        .recover(handle_rejection);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    println!("🚀 Solana HTTP Server starting on port {}", port);
    println!("🔗 Health check: http://localhost:{}/health", port);
    
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, std::convert::Infallible> {
    let error_response = if err.is_not_found() {
        ApiResponse::error("Endpoint not found".to_string())
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        ApiResponse::error("Invalid JSON in request body".to_string())
    } else {
        ApiResponse::error("Internal server error".to_string())
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&error_response),
        warp::http::StatusCode::BAD_REQUEST,
    ))
}
