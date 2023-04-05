mod entities;
mod schema;
mod setup;

// use async_graphql::{
//     http::{playground_source, GraphQLPlaygroundConfig},
//     EmptySubscription, Schema,
// };
// use async_graphql_rocket::*;

use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

// use rocket::{response::content, *};
use sea_orm::*;

use schema::*;
use setup::set_up_db;

type SchemaType = Schema<QueryRoot, MutationRoot, EmptySubscription>;

async fn index(schema: web::Data<SchemaType>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err)
    };

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
