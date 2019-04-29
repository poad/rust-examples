extern crate reqwest;

pub fn get_niconico_ranking() -> String {
    return reqwest::get("https://www.nicovideo.jp/ranking/fav/daily/all?rss=2.0&lang=ja-jp")
        .unwrap()
        .text()
        .unwrap()
}
