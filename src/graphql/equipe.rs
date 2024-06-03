use std::ops::Deref;

use actix_web::{get, post, web, HttpRequest, HttpResponse};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::ServerState;

use self::query::EquipeQueries;

pub mod query;
pub mod token;

type EquipeSchemaInner = Schema<EquipeQueries, EmptyMutation, EmptySubscription>;

#[derive(Default, Clone)]
pub struct EquipeSchema(EquipeSchemaInner);

impl Deref for EquipeSchema {
    type Target = EquipeSchemaInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[post("/equipe")]
pub async fn equipe(
    state: web::Data<ServerState>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let request = gql_request
        .into_inner()
        .data(state.client_hmac.clone())
        .data(state.db.clone())
        .data(req.headers().clone());
    state.equipe.execute(request).await.into()
}

#[get("/equipe")]
pub async fn equipe_graphiql() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/equipe").finish()))
}
