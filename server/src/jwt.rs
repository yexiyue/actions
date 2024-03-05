use anyhow::{anyhow, Context, Result};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, AppState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: i64,
    pub access_token: String,
    pub user_id: i32,
}

impl Claims {
    pub fn new(access_token: String, user_id: i32) -> Self {
        let exp = chrono::Utc::now().timestamp() + 25200;
        Self {
            exp,
            access_token,
            user_id,
        }
    }

    pub fn encode(&self, secret: &str) -> Result<String> {
        let res = jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .with_context(|| "encode jwt error")?;
        Ok(res)
    }

    pub fn decode(token: &str, secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let res = jsonwebtoken::decode(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(res.claims)
    }
}

#[async_trait]
impl FromRequestParts<AppState> for Claims {
    type Rejection = AppError;
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let res = parts
            .extensions
            .get::<Self>()
            .ok_or(AppError(StatusCode::FORBIDDEN, anyhow!("forbidden")))?;
        Ok(res.clone())
    }
}
