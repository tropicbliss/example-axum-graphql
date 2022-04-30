mod customer;

use crate::customer::Customers;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Request, Response, Schema,
};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Extension, Json, Router,
};
use customer::{CustomersSchema, Mutation, QueryRoot};

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, Mutation, EmptySubscription)
        .data(Customers::new())
        .finish();
    let app = Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema));
    println!("Playground: http://localhost:3000/graphql");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn graphql_handler(schema: Extension<CustomersSchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
