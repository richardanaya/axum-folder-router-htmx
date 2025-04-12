mod api_routes {
    // This is genearted by build.rs
    include!(concat!(env!("OUT_DIR"), "/routes.rs"));
}

#[tokio::main]
async fn main() {
    let app = api_routes::build_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
