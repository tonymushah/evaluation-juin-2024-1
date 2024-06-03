use async_graphql::{Context, Object};

use crate::{
    graphql::GetPoolConnection,
    models::temps_coureur::TempCoureur,
    modules::add_point_joueur::{AddPointJoueurEntry, AddPointJoueurModule, TempsCoureur},
};

#[derive(Debug, Clone, Copy)]
pub struct EtapeMutation(pub(super) i32);

#[Object]
impl EtapeMutation {
    pub async fn add_time(
        &self,
        ctx: &Context<'_>,
        dosard: i32,
        temps: TempsCoureur,
        point: Option<i32>,
    ) -> crate::Result<TempCoureur> {
        let entry = AddPointJoueurEntry {
            dosard,
            temps,
            point,
        };
        let etape = self.0;
        ctx.use_pool(move |mut pool| {
            Ok(AddPointJoueurModule::new(etape, &mut pool).insert_entry(&entry)?)
        })
        .await
    }
}
