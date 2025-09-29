use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use async_mpd::{MpdClient, cmd};
use std::env;

async fn get_mpd() -> Result<MpdClient, async_mpd::Error> {
  let mpd_host = env::var("MPD_HOST").unwrap_or("localhost".into());
  let mpd_port: u16 = env::var("MPD_PORT").unwrap_or("9969".into()).parse::<u16>().unwrap();
  let mut mpd = MpdClient::new();
  mpd.connect(format!("{:?}:{:?}", mpd_host, mpd_port)).await?;
  return Ok(mpd)
}

#[get("/stats")]
async fn get_stats() -> impl Responder {
  let mut mpd = get_mpd().await.unwrap();

  let response_json = mpd.stats().await.unwrap();
  HttpResponse::Ok().json(response_json)
}

#[get("/status")]
async fn get_status() -> impl Responder {
  let mut mpd = get_mpd().await.unwrap();

  let response_json = mpd.status().await.unwrap();
  HttpResponse::Ok().json(response_json)
}

#[get("/")]
async fn get_playing_song() -> impl Responder {
  let mut mpd = get_mpd().await.unwrap();

  let status = mpd.status().await.unwrap();
  let playlist = mpd.exec(cmd::PlaylistInfo).await.unwrap();
  HttpResponse::Ok().json(&playlist[status.songid.unwrap() as usize])
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init();

  println!("BasedRadio API started successfully");

  let radio_host = env::var("RADIO_API_HOST").unwrap_or("localhost".into());
  let radio_port: u16 = env::var("RADIO_API_POST").unwrap_or("9969".into()).parse::<u16>().unwrap();

  HttpServer::new(move || {
    App::new()
      .service(get_playing_song)
      .service(get_status)
      .service(get_stats)
      .wrap(Logger::default())
  })
  .bind((radio_host, radio_port))?
  .run()
  .await
}