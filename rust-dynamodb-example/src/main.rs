#[macro_use]
extern crate log;

use std::env;

extern crate rusoto_core;
extern crate rusoto_dynamodb;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, AttributeValue};
use std::collections::HashMap;
use serde::Serialize;
use actix_web::middleware::Logger;
use env_logger::Env;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

#[derive(Debug, Serialize)]
struct Comment {
    name: String,
    message: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/", web::get().to(create))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

async fn create(_req: HttpRequest) -> impl Responder {
    let endpoint = env::var("ENDPOINT").expect("ENDPOINT must be set");
    let region = Region::Custom {
        name: env::var("AWS_REGION").expect("AWS_REGION must be set").to_owned(),
        endpoint: endpoint.to_owned(),
    };
    let table = env::var("TABLE").expect("TABLE must be set");
    let comment = Comment {
        name: "test".into(),
        message: "Hello World!".into(),
    };

    let mut create_key: HashMap<String, AttributeValue> = HashMap::new();
    create_key.insert(String::from("name"), AttributeValue {
        s: Some(comment.name.to_owned()),
        ..Default::default()
    });

    let create_serials = PutItemInput {
        item: create_key,
        table_name: String::from(&table),
        ..Default::default()
    };
    let client = DynamoDbClient::new(region);
    let res = match client.put_item(create_serials).await {
        Ok(_result) => {
            HttpResponse::Ok()
                .content_type("application/json").json(&comment)
        },
        Err(error) => {
            error!("{:?}", error);
            HttpResponse::InternalServerError().finish()
        },
    };
    res
}