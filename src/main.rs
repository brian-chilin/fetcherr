mod config;
mod command;
use actix_web::{get, post, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::env;

#[post("/{route}")]
async fn post_endpoint(req: HttpRequest) -> HttpResponse {
    //println!("{:?}", req);
    let url = req.match_info().get("route").unwrap();

    if url != env::var("url").unwrap() {
        //TODO make sure unwrap on above line never crashes program (empty key string)
        //IF URL DOESN'T MATCH RETURN 404
        return HttpResponse::NotFound()
            .finish()
    } else if !(req.headers().contains_key("key") && (req.headers().get("key").unwrap().to_str().unwrap() == env::var("key").unwrap())) {
        //TODO make sure unwrap on above line never crashes program (empty key string)
        //IF THE KEY IS CORRECT CONTINUE. OTHERWISE RETURN 404
        return HttpResponse::NotFound()
            .finish()
    }

    //OTHERWISE EXECUTE
    //println!("good request received. executing command");
    command::execute();
    HttpResponse::Ok()
        .insert_header(("dummy_key", "test value"))//TODO make appropriate response header
        .finish()

}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
} //TODO remove useless endpoint

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ready up variables from config.json
    config::ready_vars();

    HttpServer::new(|| {
        App::new()
            .service(post_endpoint)
            .service(hello)
    })
        .bind(
            (env::var("ip").unwrap(), env::var("port").unwrap().parse().unwrap())
        )?
        .run()
        .await
}