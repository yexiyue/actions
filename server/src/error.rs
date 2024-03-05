use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde_json::json;

pub struct AppError(pub StatusCode, pub anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let value = json!({
            "code": self.0.as_u16(),
            "error": self.1.to_string(),
        });
        Response::builder()
            .status(self.0.as_u16())
            .header("Content-Type", "application/json")
            .body(Body::new(value.to_string()))
            .unwrap()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, err.into())
    }
}

impl AppError {
    pub fn format_err_code<T>(code: StatusCode) -> impl Fn(T) -> AppError
    where
        T: Into<anyhow::Error>,
    {
        let map_fn = move |err: T| -> AppError { AppError(code, err.into()) };
        map_fn
    }
}
