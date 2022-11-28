pub mod backend;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use backend::Worker;

/* Hello endpoint (Testing purposes) */
#[get("/hello")]
async fn hello() -> impl Responder {
    /* Returns a 200 response with 'Hello World' as body */
    HttpResponse::Ok().body("Hello World!")
}

/* Matchs endpoint, take a team name as argument */
#[get("/matchs/{team}")]
async fn get_matchs(path: web::Path<String>) -> impl Responder {
    /* Uri of the ffvb website */
    let ffvb_uri: &str = "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_calendrier.php?saison=2022/2023&codent=LIIDF&poule=RMA";

    /* Gets the name of the team from the uri */
    let team: String = path.into_inner();

    /* Build a scrapping worker */
    let worker: Worker = Worker::new(ffvb_uri).await;

    /* Scrap the matchs */
    let matchs = worker.scrap_matchs(&team);

    /* Serialize the matchs vector */
    let matchs_serialized: String = serde_json::to_string(&matchs).unwrap();

    /* Returns a 200 response with the serialized matchs as body */
    HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "http://localhost:5173"))
        .body(matchs_serialized)
}

/* Main: Creates a web server exposing endpoints */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let application = || App::new().service(hello).service(get_matchs);
    HttpServer::new(application)
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
