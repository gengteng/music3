//! Routes for the Music3 backend
//!

use crate::auth::claim::Claim;
use crate::conf::Config;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;

/// Music3 backend routes
pub fn router(config: Config) -> anyhow::Result<Router> {
    let authorizer = crate::auth::Authorizer::new(config.auth)?;
    let router = Router::new()
        .route("/", get(index))
        .nest(
            "/auth",
            Router::new()
                .route("/challenge", post(crate::auth::get_challenge))
                .route("/authorize", post(crate::auth::authorize)),
        )
        .with_state(authorizer);
    Ok(router)
}

/// Index route
pub async fn index(claim: Option<Claim>) -> impl IntoResponse {
    match claim {
        Some(claim) => format!("Hello, {}!", claim.sub),
        None => "Hello, guest!".to_string(),
    }
}
