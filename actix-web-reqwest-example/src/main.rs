use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, middleware};
use actix_web_reqwest_example::github::GitHub;
use futures::Future;
use actix_web::error::Error;

fn get_release_feed(
    req: HttpRequest
) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    GitHub::new().release_feed(req)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .route("/",
                   web::get().to_async(get_release_feed))
    })
        .bind("0.0.0.0:8000")
        .expect("Can not bind to port 8000")
        .run()
        .unwrap();
}
