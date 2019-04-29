# ニコニコ動画ランキングAPI

ニコニコ動画のランキングを取得してJSON形式で返却するAPIです。

## Usage

```$bash
cargo run
```

http://localhost:8000/

### Dockerを使用する

```$bash
docker run -it -p 8000:8000 -d poad/rust-niconico-ranking-api:latest
```

## Build

```$bash
cargo build
```

### Dockerを使用する

```$bash
docker build .
```