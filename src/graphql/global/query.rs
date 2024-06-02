use async_graphql::Object;

#[derive(Debug, Clone, Copy, Default)]
pub struct GlobalQueries;

#[Object]
impl GlobalQueries {
    pub async fn hello(&self) -> String {
        String::from("Hello from admin")
    }
}
