mod router;

use axum::{Extension, Router};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use axum::http::Method;
use sea_orm::{Database};
use tower_http::cors::CorsLayer;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = Database::connect(url).await.expect("Failed to connect to database");

    let app = Router::new()
        .merge(router::start())
        .merge(middleware())
        .layer(Extension(db));

    axum::Server::bind(&addr())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn addr() -> SocketAddr {
    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(_) => "3000".to_string(),
    };
    let port = port.parse::<u16>().expect("Invalid Port Parsed");
    println!(
        r#"
        ╔═══════════════════════════╗
        ║    Server Started on:     ║
        ║   http://localhost:{pt}   ║
        ╚═══════════════════════════╝
        "#, pt = port
    );
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port)
}

fn middleware() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(tower_http::cors::Any);

    Router::new().layer(cors)
}