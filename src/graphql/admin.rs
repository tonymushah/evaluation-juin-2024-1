use std::ops::Deref;

use actix_web::{get, post, web, HttpRequest, HttpResponse};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::ServerState;

use self::query::AdminQueries;

pub mod query;

type AdminSchemaInner = Schema<AdminQueries, EmptyMutation, EmptySubscription>;

#[derive(Default, Clone)]
pub struct AdminSchema(AdminSchemaInner);

impl Deref for AdminSchema {
    type Target = AdminSchemaInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[post("/admin")]
pub async fn admin(
    state: web::Data<ServerState>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let request = gql_request
        .into_inner()
        .data(state.db.clone())
        .data(req.headers().clone());
    state.admin.execute(request).await.into()
}

#[get("/admin")]
pub async fn admin_graphiql() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/admin").finish()))
}
