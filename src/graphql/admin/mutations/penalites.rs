use async_graphql::Context;
use diesel::{prelude::*, PgConnection};
use uuid::Uuid;

use crate::{
    graphql::GetPoolConnection, models::penalite::Penalite,
    modules::add_point_joueur::AddPointJoueurModule,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct PenaliteMuations;

impl Penalite {
    pub fn refresh_points(&self, con: &mut PgConnection) -> diesel::result::QueryResult<()> {
        AddPointJoueurModule::new(self.etape, con).attribute_points()?;
        Ok(())
    }
}

#[async_graphql::Object]
impl PenaliteMuations {
    pub async fn upsert(&self, ctx: &Context<'_>, input: Penalite) -> crate::Result<Penalite> {
        ctx.use_pool(move |mut pool| {
            use crate::schema::penalite::dsl::*;
            let res: Penalite = diesel::insert_into(penalite)
                .values(&input)
                .on_conflict(id_penalite)
                .do_update()
                .set(&input)
                .returning(Penalite::as_returning())
                .get_result(&mut pool)?;
            res.refresh_points(&mut pool)?;
            Ok(res)
        })
        .await
    }
    pub async fn remove(&self, ctx: &Context<'_>, id: Uuid) -> crate::Result<Penalite> {
        ctx.use_pool(move |mut pool| {
            use crate::schema::penalite::dsl::*;
            let res: Penalite = diesel::delete(penalite)
                .filter(id_penalite.eq(id))
                .returning(Penalite::as_returning())
                .get_result(&mut pool)?;
            res.refresh_points(&mut pool)?;
            Ok(res)
        })
        .await
    }
}
