extern crate actix_web;
extern crate mpd;

use actix_web::{App, HttpServer, Responder, web};
use mpd::Client;
// use serde::{Deserialize, Serialize};
use std::io;

async fn get_info() -> impl Responder {
  let mut conn = Client::connect("127.0.0.1:6600").unwrap(); // TODO: get from env
  println!("Status: {:?}", conn.status());
  "Hello, world!"
}

#[actix_web::main]
async fn main() -> io::Result<()> {
  HttpServer::new(|| App::new().route("/", web::get().to(get_info)))
    .bind("0.0.0.0:6969")?
    .run()
    .await
}
