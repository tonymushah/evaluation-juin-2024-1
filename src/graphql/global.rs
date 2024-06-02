pub mod query;
use std::ops::Deref;

use actix_web::{get, post, web, HttpRequest, HttpResponse};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::ServerState;

use self::query::GlobalQueries as Queries;

type GlobalSchemaInner = Schema<Queries, EmptyMutation, EmptySubscription>;

#[derive(Default, Clone)]
pub struct GlobalSchema(GlobalSchemaInner);

impl Deref for GlobalSchema {
    type Target = GlobalSchemaInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[post("/")]
pub async fn global(
    state: web::Data<ServerState>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let request = gql_request
        .into_inner()
        .data(state.db.clone())
        .data(req.headers().clone());
    state.global.execute(request).await.into()
}

#[get("/")]
pub async fn global_graphiql() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}
