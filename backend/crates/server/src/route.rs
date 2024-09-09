//! Routes for the Music3 backend
//!

use crate::conf::Config;
use axum::routing::post;
use axum::Router;

/// Music3 backend routes
pub fn router(config: Config) -> anyhow::Result<Router> {
    let authorizer = crate::auth::Authorizer::new(config.auth)?;
    let router = Router::new()
        .nest(
            "/auth",
            Router::new()
                .route("/challenge", post(crate::auth::get_challenge))
                .route("/authorize", post(crate::auth::authorize)),
        )
        .with_state(authorizer);
    Ok(router)
}
