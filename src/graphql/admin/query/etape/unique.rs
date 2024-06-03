pub mod coureur;

use async_graphql::{ComplexObject, Context, SimpleObject};
use diesel::prelude::*;

use crate::{
    graphql::{GetPoolConnection, OffsetLimit, ResultsData},
    models::{etape::Etape, Paginate},
};

use self::coureur::EtapeCoureur;

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct AdminEtape {
    #[graphql(flatten)]
    inner: Etape,
}

#[ComplexObject]
impl AdminEtape {
    pub async fn coureur(
        &self,
        ctx: &Context<'_>,
        pagination: OffsetLimit,
    ) -> crate::Result<ResultsData<EtapeCoureur>> {
        ctx.use_pool(move |mut pool| {
            use crate::view::v_temps_coureur_etape_equipe_coureur::dsl::*;
            Ok(v_temps_coureur_etape_equipe_coureur
                .select(EtapeCoureur::as_select())
                .paginate_with_param(pagination)
                .to_results_data(&mut pool)?)
        })
        .await
    }
}
