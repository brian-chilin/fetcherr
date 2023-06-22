use actix_web::{get, post, App, HttpRequest, HttpResponse, HttpServer, Responder};


#[post("/{route}")]
async fn example(req: HttpRequest) -> HttpResponse {

    let url = req.match_info().get("route").unwrap();

    if url != "secret1" {
        return HttpResponse::NotFound()
            .finish()
    }
    HttpResponse::Ok()
        .insert_header(("test1", "valueQWERTY"))
        .insert_header(("test2", "valueASDF"))
        .finish()
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(example)
            .service(hello)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}