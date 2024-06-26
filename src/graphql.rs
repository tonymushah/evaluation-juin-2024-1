use actix_web::web::block;
use async_graphql::{Context, InputObject, SimpleObject};
use diesel::{query_dsl::methods::LoadQuery, Connection, PgConnection, QueryResult};

pub mod admin;
pub mod equipe;
pub mod global;
pub mod objects;

use crate::{models::Paginated, DbPool, DbPoolConnection, ServerState};

pub trait GetPoolConnection {
    fn pool(&self) -> crate::Result<DbPoolConnection>;
    fn use_pool_blocking<D, U>(&self, usage: U) -> crate::Result<D>
    where
        U: FnOnce(DbPoolConnection) -> crate::Result<D>,
    {
        let pool = self.pool()?;
        usage(pool)
    }
    fn use_pool<D, U>(&self, usage: U) -> impl std::future::Future<Output = crate::Result<D>> + Send
    where
        Self: Sync,
        D: Send + 'static,
        U: FnOnce(DbPoolConnection) -> crate::Result<D> + Send + 'static,
    {
        async {
            let pool = self.pool()?;
            block(move || usage(pool)).await?
        }
    }
    fn use_pool_transaction<D, U>(
        &self,
        usage: U,
    ) -> impl std::future::Future<Output = crate::Result<D>> + Send
    where
        Self: Sync,
        D: Send + 'static,
        U: FnOnce(&mut DbPoolConnection) -> crate::Result<D> + Send + 'static,
    {
        async {
            let mut pool = self.pool()?;
            block(move || pool.transaction(usage)).await?
        }
    }
}

impl<'a> GetPoolConnection for Context<'a> {
    fn pool(&self) -> crate::Result<DbPoolConnection> {
        Ok(self
            .data::<DbPool>()
            .map_err(crate::Error::GraphQL)?
            .get()?)
    }
}

impl GetPoolConnection for ServerState {
    fn pool(&self) -> crate::Result<DbPoolConnection> {
        Ok(self.db.get()?)
    }
}

#[derive(Debug, Clone, Copy, InputObject)]
pub struct OffsetLimit {
    pub offset: i64,
    pub limit: i64,
}

impl Default for OffsetLimit {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 10,
        }
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(concrete(name = "EtapeResults", params(crate::models::etape::Etape)))]
#[graphql(concrete(
    name = "EtapeCoureurResults",
    params(self::admin::query::etape::unique::coureur::EtapeCoureur)
))]
#[graphql(concrete(
    name = "AdminEtapeResults",
    params(self::admin::query::etape::unique::AdminEtape)
))]
#[graphql(concrete(name = "CategorieResults", params(crate::models::categorie::Categorie)))]
#[graphql(concrete(
    name = "EquipePointResults",
    params(crate::models::equipe_point::EquipePoint)
))]
#[graphql(concrete(
    name = "CoureurPointResults",
    params(crate::models::coureur_point::CoueurPoint)
))]
#[graphql(concrete(
    name = "VEquipeCoureurResults",
    params(crate::models::equipe_coureur::VEquipeCoureur)
))]
#[graphql(concrete(name = "EquipeResults", params(crate::models::equipe::Equipe)))]
#[graphql(concrete(name = "PenalitesResults", params(crate::models::penalite::Penalite)))]
pub struct ResultsData<T>
where
    T: async_graphql::OutputType,
{
    pub data: Vec<T>,
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
}

impl<T> ResultsData<T>
where
    T: async_graphql::OutputType,
{
    pub fn map<U, B>(self, f: B) -> ResultsData<U>
    where
        U: async_graphql::ObjectType,
        B: FnMut(T) -> U,
    {
        ResultsData {
            limit: self.limit,
            offset: self.offset,
            total: self.total,
            data: self.data.into_iter().map(f).collect(),
        }
    }
    pub fn map_into<U>(self) -> ResultsData<U>
    where
        T: Into<U>,
        U: async_graphql::ObjectType,
    {
        self.map(|i| i.into())
    }
}

impl<T> Paginated<T> {
    pub fn to_results_data<'a, U>(self, conn: &mut PgConnection) -> QueryResult<ResultsData<U>>
    where
        Self: LoadQuery<'a, PgConnection, (U, i64)>,
        U: async_graphql::OutputType,
    {
        let offset = self.get_offset();
        let limit = self.get_limit();
        let (data, total) = self.load_data(conn)?;
        Ok(ResultsData {
            data,
            limit,
            offset,
            total,
        })
    }
}

#[allow(dead_code)]
pub(crate) fn get_pool(ctx: &async_graphql::Context<'_>) -> crate::Result<crate::DbPoolConnection> {
    ctx.pool()
}
