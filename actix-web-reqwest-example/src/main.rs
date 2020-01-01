use actix_web::{App, HttpRequest, HttpResponse, HttpServer, middleware, web};

use actix_web_reqwest_example::github::GitHub;

async fn get_release_feed(
    req: HttpRequest
) -> HttpResponse {
    let result = GitHub::new()
        .release_feed(req)
        .await;
    match result {
        Ok(text) =>  HttpResponse::Ok()
            .content_type("application/atom+xml,")
            .body(text),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(
                web::resource("/")
                    .route(web::get().to(get_release_feed)))
    })
        .bind("0.0.0.0:8000")
        .expect("Can not bind to port 8000")
        .run()
        .await
}
