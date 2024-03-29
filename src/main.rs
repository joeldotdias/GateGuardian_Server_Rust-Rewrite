use std::sync::Arc;

use ggserver_in_rust::{
    config::{ Config, AppState },
    database,
    router
};

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    
    let pool = database::db_connection(&config.database_url).await;

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = router::create_router(Arc::new(AppState { db: pool.clone() })).await;

    let listener = tokio::net::TcpListener::bind(&config.socket_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
