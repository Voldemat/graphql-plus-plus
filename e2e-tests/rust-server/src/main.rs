#[actix_web::post("/graphql")]
async fn hello() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| actix_web::App::new().service(hello))
        .bind((
            "0.0.0.0",
            std::env::var("PORT").unwrap().parse().unwrap()
        ))?
        .run()
        .await
}
