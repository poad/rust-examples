use std::env;

use rust_cassandra_example::cassandra::{Comment, Client, CassandraClient};
use tide::Server;
use std::sync::RwLock;


pub struct State {
    client: RwLock<Client>,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let host = format!("{}:{}", env::var("DATABASE_URL").expect("DATABASE_URL must be set"), 9042);
    let client = Client::new(host);
    client.create_keyspace("test".parse().unwrap());

    let mut app = Server::with_state(State {
        client: RwLock::new(client),
    });
    app.at("/").get( |_| async move {
        let comment = Comment {
            id: None,
            name: "test".into(),
            message: "Hello World!".into(),
        };

        let mut res = tide::Response::new(tide::StatusCode::Ok);
        res.set_body(tide::Body::from_json(&comment)?);
        Ok(res)
    });
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
