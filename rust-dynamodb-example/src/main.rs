#[macro_use]
extern crate log;

use std::env;

extern crate rusoto_core;
extern crate rusoto_dynamodb;
use actix_web::middleware::Logger;
use env_logger::Env;
use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder};
use rust_dynamodb_example::dynamodb::{Client, DynamoDBClient, Comment, CommentAccessor};
use rust_dynamodb_example::state::{DynamoClientState, State};
use actix_http::body::Body;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let endpoint = env::var("ENDPOINT").expect("ENDPOINT must be set");
    let region = env::var("AWS_REGION").expect("AWS_REGION must be set").to_owned();
    let table = env::var("TABLE").expect("TABLE must be set");

    let client = Client::new(
        (&endpoint).parse().unwrap(),
        (&region).parse().unwrap(),
        (&table).parse().unwrap());
    match (&client).find_table().await {
        Ok(true) => info!("skip the table create"),
        Ok(false) => match (&client).create_table().await {
            Ok(_) => {},
            _ => {}
        },
        _ => panic!("DynamoDB Access error")
    }

    HttpServer::new(move || {
        App::new()
            .data(DynamoClientState::new(client.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/", web::get().to( list))
            .route("/{name}", web::get().to( get_handle))
            .route("/{name}/", web::get().to( get_handle))
            .route("/", web::post().to( post_handle))
            .route("/", web::put().to( post_handle))
            .route("/{name}", web::delete().to( delete_handle))
            .route("/{name}/", web::delete().to( delete_handle))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

async fn list(_: HttpRequest, state: web::Data<DynamoClientState>) -> impl Responder {
    match &state.client().list_items().await {
        Ok(items) => HttpResponse::Ok().json(items),
        _ => HttpResponse::InternalServerError().body(Body::None)
    }
}

async fn get_handle(req: HttpRequest, state: web::Data<DynamoClientState>) -> impl Responder {
    match req.match_info().get("name") {
        Some(name) => {
            match &state.client().get_item(name.parse().unwrap()).await {
                Ok(item) => match item {
                    Some(value) => HttpResponse::Ok().json(value),
                    _ => HttpResponse::NotFound().body(Body::Empty)
                },
                _ => HttpResponse::InternalServerError().body(Body::None)
            }
        },
        None => HttpResponse::BadRequest().body(Body::None)
    }
}

async fn post_handle(comment: web::Json<Comment>, state: web::Data<DynamoClientState>) -> impl Responder {
    match &state.client().put_item(comment.name(), comment.message()).await {
        Ok(key) => HttpResponse::Ok().json(key),
        _ => HttpResponse::InternalServerError().body(Body::None)
    }
}

async fn delete_handle(req: HttpRequest, state: web::Data<DynamoClientState>) -> impl Responder {
    match req.match_info().get("name") {
        Some(name) => {
            match &state.client().delete_item(name.parse().unwrap()).await {
                Ok(result) => if result.clone() {
                    HttpResponse::NoContent().body(Body::None)
                } else {
                    HttpResponse::NotFound().body(Body::None)
                },
                _ => HttpResponse::InternalServerError().body(Body::None)
            }
        },
        None => HttpResponse::BadRequest().body(Body::None)
    }
}
