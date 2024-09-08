//! # JSON Web Token Claim

use crate::auth::error::Error;
use crate::auth::Authorizer;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use jsonwebtoken::get_current_timestamp;
use serde::{Deserialize, Serialize};

/// JSON Web Token Claim
#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    /// Subject
    pub sub: String,
    /// Expiration
    pub exp: usize,
}

impl Claim {
    /// Create a new claim
    pub fn create(sub: impl Into<String>, duration_sec: usize) -> Self {
        Self {
            sub: sub.into(),
            exp: get_current_timestamp() as usize + duration_sec,
        }
    }

    /// Check if the token is expired
    pub fn expired(&self) -> bool {
        self.exp < get_current_timestamp() as usize
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for Claim
where
    Authorizer: FromRef<S>,
    S: Sync + Send,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(Error::JwtNotProvidedOrInvalid)?;
        let authorizer = Authorizer::from_ref(state);
        authorizer.jwt.verify(bearer.token())
    }
}
