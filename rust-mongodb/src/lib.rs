use mongodb::{Client, options::{ClientOptions, StreamAddress}};
use mongodb::error::Result;
use mongodb::results::{InsertOneResult, DeleteResult};
use bson::{doc, bson, Document};

pub fn connect_mongodb() -> Client {
    let options = ClientOptions::builder()
        .hosts(vec![
            StreamAddress {
                hostname: "mongo".into(),
                port: Some(27017),
            }
        ])
        .build();

    Client::with_options(options)
        .expect("Failed to initialize standalone client.")
}

pub fn create_post(client: &Client, title: &str, body: &str) -> InsertOneResult {
    let coll = client.database("posts").collection("post");
    coll.insert_one(doc!{ "title": title, "body": body }, None).unwrap()
}

pub fn all_posts(client: Client) -> Vec<Document> {
    let coll = client.database("posts").collection("post");
    let cursor = coll.find(None, None).unwrap();
    cursor
        .filter(|item| {
            match item {
                Ok(_) => true,
                _ => false
            }
        })
        .map(|item| {
            match item {
                Ok(item) => item,
                _ => panic!("")
            }
        })
        .collect()
}

pub fn get_post(client: &Client, title: String) -> Option<Document> {
    let coll = client.database("posts").collection("post");
    return match coll.find_one(Some(doc!{"title": title}), None) {
            Ok(item) => item,
            _ => None
        };
}

pub fn delete_post(client: &Client, title: String) -> Result<DeleteResult> {
    let coll = client.database("posts").collection("post");
    return coll.delete_one(doc!{"title": title.to_owned()}, None);
}
