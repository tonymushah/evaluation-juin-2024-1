use std::ops::Deref;

use async_graphql::{Context, Object};
use diesel::{delete, insert_into, prelude::*};
use jwt::SignWithKey;
use uuid::Uuid;

use crate::{
    graphql::GetPoolConnection,
    models::{equipe_coureur::VEquipeCoureur, etape::Etape, temps_coureur::TempCoureur},
};

use super::{
    query::etapes::EtapeQueries,
    token::{ClientHmac, ExtractCurrentEquipe},
};

#[derive(Debug, Clone, Default, Copy)]
pub struct EquipeMutations;

#[Object]
impl EquipeMutations {
    pub async fn login(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> crate::Result<String> {
        let sha = ctx.data::<ClientHmac>()?.clone();
        ctx.use_pool(move |mut pool| {
            use crate::schema::equipe::dsl::*;
            Ok(equipe
                .select(id_equipe)
                .filter(pseudo.eq(username))
                .filter(mot_passe.eq(password))
                .get_result::<Uuid>(&mut pool)?
                .sign_with_key(sha.deref())?)
        })
        .await
    }
    pub async fn ajouter_joueur_to_etape(
        &self,
        ctx: &Context<'_>,
        etape_: i32,
        joueur: i32,
    ) -> crate::Result<TempCoureur> {
        let limit = ctx
            .use_pool(move |mut pool| Ok(Etape::get(etape_, &mut pool)?))
            .await?
            .nb_coureur_par_equipe as usize;
        let js = EtapeQueries.joueurs(ctx, etape_).await?;
        if js.len() < limit {
            if !js.iter().any(|ec| ec.coureur == joueur) {
                // let current = ctx.get_current_equipe()?;
                ctx.use_pool(move |mut pool| {
                    let j = VEquipeCoureur::by_coureur(joueur, &mut pool)?;
                    use crate::schema::temps_coureur::dsl::*;
                    let temp = TempCoureur {
                        id_temps_coureur: Uuid::new_v4(),
                        temps: None,
                        points: None,
                        equipe_coureur: j.id_equipe_coureur,
                        etape: etape_,
                    };
                    insert_into(temps_coureur)
                        .values(&temp)
                        .execute(&mut pool)?;
                    Ok(temp)
                })
                .await
            } else {
                Err(crate::Error::AlreadyInEtape)
            }
        } else {
            Err(crate::Error::LimitCoureurExecded)
        }
    }
    pub async fn remove_joueur_to_etape(
        &self,
        ctx: &Context<'_>,
        etape_: i32,
        joueur: i32,
    ) -> crate::Result<TempCoureur> {
        let _current = ctx.get_current_equipe()?;
        ctx.use_pool(move |mut pool| {
            let j = VEquipeCoureur::by_coureur(joueur, &mut pool)?;
            use crate::schema::temps_coureur::dsl::*;
            let temps_: TempCoureur = temps_coureur
                .select(TempCoureur::as_select())
                .filter(etape.eq(etape_))
                .filter(equipe_coureur.eq(j.id_equipe_coureur))
                .get_result(&mut pool)?;
            if temps_.temps.is_none() {
                Ok(delete(temps_coureur)
                    .filter(equipe_coureur.eq(j.id_equipe_coureur))
                    .filter(etape.eq(etape_))
                    .returning(TempCoureur::as_select())
                    .get_result(&mut pool)?)
            } else {
                Err(crate::Error::AlreadyDone)
            }
        })
        .await
    }
}
