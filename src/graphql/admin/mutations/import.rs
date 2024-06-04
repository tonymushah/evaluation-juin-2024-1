use std::io::BufReader;

use async_graphql::{Context, Object, Upload};
use csv::Reader;
use diesel::{insert_into, prelude::*};

use crate::{
    graphql::GetPoolConnection,
    models::{etape::Etape, points::Points, temps_coureur::TempCoureur},
    modules::{
        add_point_joueur::attribute_points_to_etapes,
        imports::{etape::EtapeCSVDATA, points::PointsCSVData, resultat::ResultatCSV},
    },
};

pub struct ImportMutations;

#[Object]
impl ImportMutations {
    pub async fn points(&self, ctx: &Context<'_>, file: Upload) -> crate::Result<Vec<Points>> {
        let file = BufReader::new(file.value(ctx)?.content);
        let points_data = PointsCSVData::read(Reader::from_reader(file))
            .into_iter()
            .map(|p| p.into())
            .collect::<Vec<Points>>();
        ctx.use_pool(move |mut pool| {
            use crate::schema::points::dsl::*;
            Ok(insert_into(points)
                .values(&points_data)
                .on_conflict(rang)
                .do_nothing()
                .returning(Points::as_returning())
                .get_results(&mut pool)?)
        })
        .await
    }
    pub async fn etapes(&self, ctx: &Context<'_>, file: Upload) -> crate::Result<Vec<Etape>> {
        let buf_read = BufReader::new(file.value(ctx)?.content);
        let etapes_data: Vec<Etape> = EtapeCSVDATA::read(Reader::from_reader(buf_read))
            .into_iter()
            .map(|p| p.into())
            .collect();
        ctx.use_pool(move |mut pool| {
            use crate::schema::etape::dsl::*;
            Ok(insert_into(etape)
                .values(&etapes_data)
                .on_conflict(rang)
                .do_nothing()
                .returning(Etape::as_returning())
                .get_results(&mut pool)?)
        })
        .await
    }
    pub async fn resultats(
        &self,
        ctx: &Context<'_>,
        file: Upload,
    ) -> crate::Result<Vec<TempCoureur>> {
        let buf_read = BufReader::new(file.value(ctx)?.content);
        let data = ResultatCSV::read(Reader::from_reader(buf_read));
        // let data_len = data.len();
        let mut etapes = data.iter().map(|r| r.etape_rang).collect::<Vec<_>>();
        etapes.dedup();

        ctx.use_pool(move |mut pool| {
            let _temps = data
                .into_iter()
                .flat_map(|res| res.insert(&mut pool))
                .map(|res| res.id_temps_coureur)
                .collect::<Vec<_>>();
            Ok(attribute_points_to_etapes(&mut pool, &etapes)?)
        })
        .await
    }
}
