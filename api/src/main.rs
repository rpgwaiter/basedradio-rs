use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use async_mpd::{MpdClient, cmd};


#[get("/stats")]
async fn get_stats() -> impl Responder {
  let mut mpd = MpdClient::new();
  mpd.connect("localhost:6600").await.unwrap();

  let response_json = mpd.stats().await.unwrap();
  HttpResponse::Ok().json(response_json)
}

#[get("/status")]
async fn get_status() -> impl Responder {
  let mut mpd = MpdClient::new();
  mpd.connect("localhost:6600").await.unwrap();

  let response_json = mpd.status().await.unwrap();
  HttpResponse::Ok().json(response_json)
}

#[get("/")]
async fn get_playing_song() -> impl Responder {
  let mut mpd = MpdClient::new();
  mpd.connect("localhost:6600").await.unwrap();

  let status = mpd.status().await.unwrap();
  let playlist = mpd.exec(cmd::PlaylistInfo).await.unwrap();
  HttpResponse::Ok().json(&playlist[status.songid.unwrap() as usize])
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init();

  println!("BasedRadio API started successfully");

  HttpServer::new(move || {
    App::new()
      .service(get_playing_song)
      .service(get_status)
      .service(get_stats)
      .wrap(Logger::default())
  })
  .bind(("0.0.0.0", 8000))?
  .run()
  .await
}