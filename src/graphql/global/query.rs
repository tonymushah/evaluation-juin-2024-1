pub mod classement;

use async_graphql::Object;

use self::classement::ClassementQueries;

#[derive(Debug, Clone, Copy, Default)]
pub struct GlobalQueries;

#[Object]
impl GlobalQueries {
    pub async fn hello(&self) -> String {
        String::from("Hello from admin")
    }
    pub async fn classements(&self) -> ClassementQueries {
        ClassementQueries
    }
}
