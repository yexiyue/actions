use async_graphql::Object;

#[derive(Debug, Default)]
pub struct ActionMutation;

#[Object]
impl ActionMutation {
    async fn create_action(&self) -> String {
        "create".into()
    }
}
