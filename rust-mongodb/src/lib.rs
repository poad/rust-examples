use futures::stream::StreamExt;
use mongodb::bson::{doc, Document};
use mongodb::error::Result;
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::{options::{ClientOptions, StreamAddress}, Client, Cursor};

pub fn connect_mongodb() -> Client {
    let options = ClientOptions::builder()
        .hosts(vec![StreamAddress {
            hostname: "mongo".into(),
            port: Some(27017),
        }])
        .build();

    Client::with_options(options).expect("Failed to initialize standalone client.")
}

pub async fn create_post(client: &Client, title: &str, body: &str) -> InsertOneResult {
    let coll = client.database("posts").collection("post");
    coll.insert_one( doc! { "title": title, "body": body }, None)
        .await
        .unwrap()
}

pub async fn all_posts(client: Client) -> Vec<Document> {
    let cursor: Cursor = client
        .database("posts")
        .collection("post")
        .find(None, None)
        .await
        .unwrap();

    let result = cursor
        .map(|item| {
            item.unwrap()
        })
        .collect();

    result.await
}

pub async fn get_post(client: &Client, title: String) -> Option<Document> {
    let coll = client.database("posts").collection("post");
    let result =  match coll.find_one(Some( doc! {"title": title}), None).await {
        Ok(item) => item,
        _ => None,
    };
    return result;
}

pub async fn delete_post(client: &Client, title: String) -> Result<DeleteResult> {
    let coll = client.database("posts").collection("post");
    return coll
        .delete_one(doc! {"title": title}, None)
        .await;
}
