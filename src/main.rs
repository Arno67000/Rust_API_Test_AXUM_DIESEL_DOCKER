use axum::routing::get;
use axum::Router;
use colored::Colorize;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod controllers;
mod handlers;
mod models;
mod providers;
mod repositories;
mod routers;
mod schema;

use crate::config::environment::Environment;
use crate::providers::database::DB;

use crate::providers::migrations::run_migration;
use crate::routers::{players_router, teams_router};

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

    // Running pending migrations
    run_migration(&db);

    // Creating app
    let app = Router::new()
        .route("/", get(welcome()))
        .nest("/players", players_router::routes())
        .nest("/teams", teams_router::routes())
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
