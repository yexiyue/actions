use async_graphql::MergedObject;

mod actions;
mod history;

#[derive(Debug, Default, MergedObject)]
pub struct Mutation(actions::ActionMutation);
