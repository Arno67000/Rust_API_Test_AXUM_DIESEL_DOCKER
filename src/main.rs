use axum::{routing::get, Router};
use colored::Colorize;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod controllers;
mod providers;
mod repositories;
mod schema;

use crate::config::environment::Environment;
use crate::controllers::users_controller::{create_user, get_all_users};
use crate::providers::database::DB;

#[tokio::main]
async fn main() {
    // Getting environment
    let env: Environment = Environment::get();
    println!("{:?}", env);

    // Setting up logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(env.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = DB::new(env.database_url);

    // Creating app
    let app = Router::new()
        .route("/", get(welcome()))
        .route("/users", get(get_all_users).post(create_user))
        .with_state(db)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr = SocketAddr::from((env.host, env.port));
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => panic!("{}{:?}", "TCP Listener bind failed : ".red(), e),
    };
    println!(
        "{}{}",
        "Server ready & listening on : ".blue().italic(),
        addr.to_string().blue().italic()
    );

    let service = axum::serve(listener, app.into_make_service());

    if let Err(e) = service.await {
        eprintln!("{}{:?}", "Error: ".red(), e);
    }
}

fn welcome() -> String {
    "Welcome to RUST API".to_string()
}
