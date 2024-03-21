use async_graphql::MergedObject;

mod history;
mod user;

#[derive(Debug, MergedObject, Default)]
pub struct Query(user::UserQuery);
