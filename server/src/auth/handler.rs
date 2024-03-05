use crate::{
    error::AppError,
    jwt::Claims,
    service::{SessionService, UserService},
    AppState,
};

use super::OAuth;
use anyhow::{anyhow, Context, Result};
use axum::{
    extract::{Query, State},
    http::{header, status::StatusCode, HeaderMap},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar, SameSite};
use chrono::{TimeDelta, Utc};
use oauth2::TokenResponse;
use serde::{Deserialize, Serialize};

pub async fn login(oauth: OAuth, jar: PrivateCookieJar) -> impl IntoResponse {
    let (url, csrf_token) = oauth.generate_oauth_url();
    let cookie = Cookie::build(("csrf_token", csrf_token.secret().to_string()))
        .same_site(SameSite::Lax)
        .secure(true)
        .http_only(true)
        .build();
    (jar.add(cookie), Redirect::to(&url.to_string()))
}

#[derive(Deserialize, Serialize)]
pub struct CallbackParams {
    code: String,
    state: String,
}

pub async fn callback(
    Query(CallbackParams { code, state }): Query<CallbackParams>,
    State(AppState {
        req,
        coon,
        secret_store,
        ..
    }): State<AppState>,
    header: HeaderMap,
    oauth: OAuth,
    jar: PrivateCookieJar,
) -> Result<impl IntoResponse, AppError> {
    let csrf = jar
        .get("csrf_token")
        .ok_or(AppError::format_err_code(StatusCode::BAD_REQUEST)(anyhow!(
            "csrf token not found"
        )))?;

    if state != csrf.value() {
        return Err(AppError(
            StatusCode::BAD_REQUEST,
            anyhow!("state does not match"),
        ));
    }
    let user_agent = header.get(header::USER_AGENT).unwrap();
    // 交换获取access_token
    let res = oauth.exchange_code(code).await?;
    let access_token = res.access_token().secret();
    let refresh_token = res
        .refresh_token()
        .expect("refresh token not found")
        .secret();

    let seconds = res.expires_in().expect("expires_in not found");
    let expires_at =
        Utc::now() + TimeDelta::from_std(seconds).expect("expires_in to time delta error");

    // 使用获取的access_token获取用户信息
    let user: serde_json::Value = req
        .get("https://api.github.com/user")
        .bearer_auth(access_token)
        .header(reqwest::header::USER_AGENT, user_agent.to_str().unwrap())
        .send()
        .await?
        .json()
        .await?;
    let user_id = user["id"].as_i64().unwrap();
    let username = user["login"].as_str().unwrap();
    let avatar_url = user["avatar_url"].as_str().unwrap();

    // 如果用户不存在就创建
    let user = UserService::create_or_find(
        &coon,
        entity::user::Model {
            id: user_id as i32,
            username: username.to_owned(),
            avatar_url: avatar_url.to_owned(),
            create_at: chrono::Utc::now().into(),
        },
    )
    .await?;

    // 创建会话记录保存refresh_token和access_token
    SessionService::create_or_update_session(
        &coon,
        entity::session::Model {
            id: 0,
            user_id: user.id,
            access_token: access_token.into(),
            refresh_token: refresh_token.into(),
            expires_at: expires_at.into(),
        },
    )
    .await?;

    // 获取jwt密钥
    let jwt_secret = secret_store
        .get("JWT_SECRET")
        .with_context(|| "get jwt secret error")?;

    // 生成jwt token
    let token = Claims::new(access_token.to_string(), user_id as i32).encode(&jwt_secret)?;

    let token_cookie = Cookie::build(("token", token))
        .http_only(true)
        .path("/")
        .secure(true)
        .max_age(cookie::time::Duration::seconds_f64(seconds.as_secs_f64()))
        .build();

    let user_id_cookie = Cookie::build(("user_id", user_id.to_string()))
        .http_only(true)
        .path("/")
        .secure(true)
        .build();

    Ok((jar.add(token_cookie).add(user_id_cookie), Redirect::to("/")))
}
