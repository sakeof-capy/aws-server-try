use actix_web::{
    get, 
    post, 
    web, 
    web::Json,
    App, 
    HttpResponse, 
    HttpServer, 
    Responder
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("ПАРІВЄТ, МАТЯПУ!")
}

#[get("/json")]
async fn json() -> impl Responder {
    Json("SOME JSON STRING".to_string())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(json)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
