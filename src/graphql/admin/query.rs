pub mod etape;

use self::etape::EtapeQueries;

use async_graphql::Object;

#[derive(Debug, Clone, Copy, Default)]
pub struct AdminQueries;

#[Object]
impl AdminQueries {
    pub async fn hello(&self) -> String {
        String::from("Hello from admin")
    }
    pub async fn etape(&self) -> EtapeQueries {
        EtapeQueries
    }
}
