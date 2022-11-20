pub mod backend;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use backend::Worker;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/matchs/{team}")]
async fn get_matchs(path: web::Path<String>) -> impl Responder {
    let ffvb_uri: &str = "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_calendrier.php?saison=2022/2023&codent=LIIDF&poule=RMA";
    let team: String = path.into_inner();
    let worker: Worker = Worker::new(ffvb_uri).await;
    let matchs_serialized: String = serde_json::to_string(&worker.scrap_matchs(&team)).unwrap();
    HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "http://localhost:5173"))
        .body(matchs_serialized)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let application = || App::new().service(hello).service(get_matchs);
    HttpServer::new(application)
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
