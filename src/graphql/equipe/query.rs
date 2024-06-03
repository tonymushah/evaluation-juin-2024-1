pub mod etapes;

use async_graphql::Object;

use self::etapes::EtapeQueries;

#[derive(Debug, Clone, Copy, Default)]
pub struct EquipeQueries;

#[Object]
impl EquipeQueries {
    pub async fn hello(&self) -> String {
        String::from("Hello from admin")
    }
    pub async fn etape(&self) -> EtapeQueries {
        EtapeQueries
    }
}
