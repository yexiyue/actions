use crate::{error::AppError, jwt::Claims, service::SessionService, AppState};
use anyhow::{anyhow, Context};
use axum::{
    extract::{Request, State},
    http::{header, HeaderValue, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::PrivateCookieJar;
use chrono::{TimeDelta, Utc};
use cookie::Cookie;
use oauth2::TokenResponse;

pub async fn claims_middleware(
    jar: PrivateCookieJar,
    State(AppState {
        secret_store,
        auth,
        coon,
        ..
    }): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    // 获取token

    let token = jar.get("token");
    if let Some(token) = token {
        let jwt_secret = secret_store
            .get("JWT_SECRET")
            .with_context(|| "get jwt secret error")?;
        // 解析token
        match Claims::decode(token.value(), jwt_secret.as_str()) {
            Ok(claims) => {
                req.extensions_mut().insert(claims);
                let response = next.run(req).await;
                return Ok(response);
            }
            Err(err) => match err.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    let user_id = jar
                        .get("user_id")
                        .ok_or(anyhow!("undefined authorization cookie"))
                        .map_err(AppError::format_err_code(StatusCode::FORBIDDEN))?
                        .value()
                        .parse::<i32>()?;
                    let session = SessionService::find_session_by_user_id(&coon, user_id)
                        .await?
                        .ok_or(AppError(
                            StatusCode::FORBIDDEN,
                            anyhow!("session not found"),
                        ))?;
                    let oauth_res = auth.refresh_token(session.refresh_token).await?;
                    let access_token = oauth_res.access_token().secret();
                    let refresh_token = oauth_res
                        .refresh_token()
                        .expect("refresh token not found")
                        .secret();

                    let seconds = oauth_res.expires_in().expect("expires_in not found");
                    let expires_at = Utc::now()
                        + TimeDelta::from_std(seconds).expect("expires_in to time delta error");

                    // 创建会话记录保存refresh_token和access_token
                    SessionService::create_or_update_session(
                        &coon,
                        entity::session::Model {
                            id: 0,
                            user_id,
                            access_token: access_token.into(),
                            refresh_token: refresh_token.into(),
                            expires_at: expires_at.into(),
                        },
                    )
                    .await?;

                    // 生成jwt token
                    let claims: Claims = Claims::new(access_token.to_string(), user_id as i32);
                    let token = claims.encode(&jwt_secret)?;

                    req.extensions_mut().insert(claims);
                    let mut response = next.run(req).await;

                    let token_cookie = Cookie::build(("token", token))
                        .http_only(true)
                        .path("/")
                        .secure(true)
                        .max_age(cookie::time::Duration::seconds_f64(seconds.as_secs_f64()))
                        .build();

                    response.headers_mut().insert(
                        header::SET_COOKIE,
                        HeaderValue::from_str(&token_cookie.to_string()).unwrap(),
                    );
                    return Ok(response);
                }
                _ => {
                    return Err(AppError(StatusCode::FORBIDDEN, err.into()));
                }
            },
        };
    }
    let response = next.run(req).await;
    return Ok(response);
}
