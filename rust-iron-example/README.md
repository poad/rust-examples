The Rust Iron example
------------------------

### How to run?

```
docker run --name postgres -e POSTGRES_PASSWORD=test -p 5432:5432 -d postgres:alpine
cd rust-iron-example/src
cargo install diesel_cli --no-default-features --features postgres
echo DATABASE_URL=postgres://postgres:test@127.0.0.1/diesel_demo > .env
diesel setup
#diesel migration generate create_posts
diesel migration run
cargo run --bin rust_iron_example
```

### TODO

 - POST support

### reference
 - [Rustに入門してIron触ってみた。](https://qiita.com/shamisonn/items/24fe203ca4fd610e4a25#簡単にgetpostをしてみる)
 - [iron/body-parser](https://github.com/iron/body-parser#example)
