use async_graphql::Object;

#[derive(Debug, Clone, Copy, Default)]
pub struct EquipeQueries;

#[Object]
impl EquipeQueries {
    pub async fn hello(&self) -> String {
        String::from("Hello from admin")
    }
}
