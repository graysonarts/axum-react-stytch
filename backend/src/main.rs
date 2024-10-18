use std::net::SocketAddr;

use axum::Router;
#[cfg(feature = "git-version")]
use const_format::concatcp;
use infra::log::LogRequest;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod app_state;
mod auth;
mod infra;

const NAME: &str = env!("CARGO_PKG_NAME");
#[cfg(not(feature = "git-version"))]
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(feature = "git-version")]
const CARGO_VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(feature = "git-version")]
const GIT_VERSION: &str = git_version::git_version!(args = ["--always", "--dirty=-modified"]);
#[cfg(feature = "git-version")]
const VERSION: &str = concatcp!(CARGO_VERSION, " (", GIT_VERSION, ")");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    tracing::info!("{NAME} v{VERSION}");

    let fallback_asset = "./public".to_owned();
    let asset_svc = ServeDir::new(fallback_asset);
    let app_state = app_state::init().await?;

    let app = Router::new()
        .nest("/auth", auth::routes())
        // eg: You have an auth middleware function:
        //    .route_layer(from_fn_with_state(
        //      Arc::clone(&app_state),
        //      auth::auth_required,
        //     ))
        // Insecure routes start here
        .layer(CorsLayer::permissive()) // TODO: Make this specific
        .layer(
            TraceLayer::new_for_http()
                .on_request(LogRequest::default())
                .on_response(LogRequest::default()),
        )
        .with_state(app_state)
        .fallback_service(asset_svc);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on http://{}", addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
