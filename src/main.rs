mod api_routes {
    // This is generated by build.rs
    include!(concat!(env!("OUT_DIR"), "/routes.rs"));
}

use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = api_routes::build_router()
        .nest_service("/", ServeDir::new("public"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
