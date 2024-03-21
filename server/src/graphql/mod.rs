use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::{jwt::Claims, AppState};
mod auth_guard;
mod mutation;
mod query;
pub use auth_guard::AuthGuard;

use self::{mutation::Mutation, query::Query};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema() -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish()
}

// http入口
pub async fn graphql_handler(
    State(state): State<AppState>,
    claims: Option<Claims>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    req = req
        .data(claims)
        .data(state.auth)
        .data(state.coon);

    state.schema.execute(req).await.into()
}

// graphql IDE
pub async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/api/graphql").finish())
}
