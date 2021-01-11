use std::env;

use rust_cassandra_example::cassandra::{Client, CassandraClient};
use tide::Request;


#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();

    app.at("/").get(index);

    app.listen("0.0.0.0:8080").await.unwrap();
    Ok(())
}

async fn index(_: Request<()>) -> tide::Result {
    let host = format!("{}:{}", env::var("DATABASE_URL").expect("DATABASE_URL must be set"), 9042);
    let client = Client::new(host);
    client.create_keyspace("test".parse().unwrap());
    client.create_table("test.test".parse().unwrap(), "(id text PRIMARY KEY, name text, message text)".parse().unwrap());

    client.select("SELECT * from test.test".parse().unwrap());

    Ok("".into())
}