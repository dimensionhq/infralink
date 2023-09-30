use super::schema::Query;

use actix_web::{web, HttpRequest, HttpResponse};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sqlx::{Pool, Postgres};

pub async fn graphql(
    pool: web::Data<Pool<Postgres>>,
    schema: web::Data<Schema<Query, EmptyMutation, EmptySubscription>>,
    req: HttpRequest,
    gql: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql.into_inner();

    request = request.data(pool.clone());
    request = request.data(req.headers().clone());

    return schema.execute(request).await.into();
}

pub async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish())
}
