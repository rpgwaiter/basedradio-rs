mod types;

use actix_web::middleware::Logger;
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use async_mpd::{MpdClient, cmd};
use icecast_stats::IcecastStatsRoot;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::{env, fs}; // TODO: async fs
use types::{ApiResponse, MetaInfo, MoreInfo, RadioStatus, Song, Updates};
use urlencoding::encode;

fn good_encode(s: &str) -> String {
  encode(s).replace("%2F", "/")
}

fn get_meta(file: &str) -> MetaInfo {
  let mut sp = file.split("/");

  let system = sp.next().unwrap();
  let game = sp.next().unwrap();

  return MetaInfo {
    game: game.to_string(),
    system: system.to_string(),
  };
}

fn get_song_parent(file: &str) -> Option<PathBuf> {
  let music_root = env::var("RADIO_MUSIC_DIR").unwrap_or("/Music".into());
  let song_full_path: PathBuf = Path::new(&music_root).join(file);
  song_full_path.parent().map(|p| p.to_path_buf())
}

fn get_download_link(file: &str) -> String {
  let filehost_url = env::var("RADIO_FILEHOST_URL").unwrap_or("http://localhost:6969".into());
  let encoded = encode(file).into_owned().replace("%2F", "/");
  return format!("{filehost_url}/{encoded}");
}

// Takes the file path from an mpd status
fn get_cover(file: &str, target: &str) -> Option<String> {
  let regex = Regex::new(&format!(r"(?i)^{}\.(gif|jpeg|jpg|png|webp)$", target)).unwrap();
  let filehost_url = env::var("RADIO_FILEHOST_URL").unwrap_or("http://localhost:6969".into());
  let music_root = env::var("RADIO_MUSIC_DIR").unwrap_or("/Music".into());

  let song_parent = get_song_parent(&file).unwrap();
  let files = fs::read_dir(&song_parent).unwrap();

  for entry in files.flatten() {
    if let Ok(file_name) = entry.file_name().into_string() {
      if regex.is_match(&file_name) {
        let path = Path::new(
          &song_parent
            .to_str() // "/Music/system/game"
            .unwrap()
            .replace(&music_root, ""), // "/system/game"
        )
        .join(file_name); // "/system/game/cover.png"

        // Maybe we should join paths again for this idk
        let path_str = path.to_str().unwrap();
        let encoded = good_encode(path_str);

        return Some(format!("{filehost_url}{encoded}"));
      }
    }
  }
  None
  // return format!("{filehost_url}/{target}.png");
}

// Takes the file path from an mpd status
fn get_more_info(file: &str) -> MoreInfo {
  let song_parent = get_song_parent(&file).unwrap();
  let potential_json = song_parent.join("info.json");

  if let Ok(raw_file) = fs::read_to_string(potential_json) {
    if let Ok(info) = serde_json::from_str::<MoreInfo>(&raw_file) {
      return info;
    };
  };
  MoreInfo::new()
}

// TODO: err handle
async fn get_icecast_info() -> Result<IcecastStatsRoot, reqwest::Error> {
  let icecast_url =
    env::var("ICECAST_JSON_URL").unwrap_or("https://cast.based.radio/status-json.xsl".into());

  return reqwest::get(icecast_url)
    .await
    .unwrap()
    .json::<IcecastStatsRoot>()
    .await;
}

#[get("/updates")]
async fn get_updates() -> impl Responder {
  let update_url = env::var("RADIO_UPDATES_URL").unwrap_or(
    "https://raw.githubusercontent.com/rpgwaiter/basedradio-rs/refs/heads/main/updates.json".into(),
  );

  let ret = reqwest::get(update_url)
    .await
    .unwrap()
    .json::<Updates>()
    .await
    .unwrap();

  HttpResponse::Ok().json(ret)
}

async fn get_mpd() -> Result<MpdClient, async_mpd::Error> {
  let mpd_host = env::var("MPD_HOST").unwrap_or("localhost".into());
  let mpd_port: u16 = env::var("MPD_PORT")
    .unwrap_or("6600".into())
    .parse::<u16>()
    .unwrap();
  let mut mpd = MpdClient::new();
  let mpd_addr = format!("{mpd_host}:{mpd_port}");

  mpd.connect(mpd_addr).await?;
  return Ok(mpd);
}

#[get("/more-info")]
async fn more_info() -> impl Responder {
  let mut mpd = get_mpd().await.unwrap();
  let status = mpd.status().await.unwrap();
  let playlist = mpd.exec(cmd::PlaylistInfo).await.unwrap();
  let current_song = &playlist[status.song.unwrap() as usize];

  let more_info = get_more_info(&current_song.file);

  HttpResponse::Ok().json(more_info)
}

#[get("/")]
async fn get_playing_song() -> impl Responder {
  let filehost_url = env::var("RADIO_FILEHOST_URL").unwrap_or("http://localhost:6969".into());
  let mut mpd = get_mpd().await.unwrap();

  let status = mpd.status().await.unwrap();
  let playlist = mpd.exec(cmd::PlaylistInfo).await.unwrap();
  let current_song = &playlist[status.song.unwrap() as usize];
  let meta = get_meta(&current_song.file);
  let status = mpd.status().await.unwrap();

  let icecast_info = get_icecast_info().await.unwrap();
  let file = &current_song.file;

  // Deal with titles not being in the metadata
  let title = current_song
    .clone()
    .title
    .unwrap_or(String::from(current_song.file.rsplit_once("/").unwrap().1));

  HttpResponse::Ok().json(ApiResponse {
    song: Song {
      album: current_song.clone().album,
      artist: current_song.clone().artist,
      background: get_cover(&file, "bg"),
      cover: get_cover(&file, "cover")
        .unwrap_or(format!("{filehost_url}/{}/cover.png", meta.system)),
      file: current_song.clone().file,
      download_link: get_download_link(&file),
      game: meta.game,
      system: meta.system,
      title: Some(title),
    },
    status: RadioStatus {
      elapsed: status.elapsed.unwrap().as_secs(),
      duration: status.duration.unwrap().as_secs(),
      total_songs: status.playlistlength,
      listeners: icecast_info // TODO: map over an iter
        .icestats
        .sources_vec()
        .pop()
        .unwrap()
        .listeners()
        .unwrap_or(0),
    },
    more_info: get_more_info(&file),
  })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init();

  println!("BasedRadio API started successfully");

  let radio_host = env::var("RADIO_API_HOST").unwrap_or("localhost".into());
  let radio_port: u16 = env::var("RADIO_API_PORT")
    .unwrap_or("9969".into())
    .parse::<u16>()
    .unwrap();

  println!("radio host: {:?}", radio_host);
  println!("radio port: {:?}", radio_port);

  HttpServer::new(move || {
    App::new()
      .service(more_info)
      .service(get_playing_song)
      .service(get_updates)
      .wrap(Logger::default())
  })
  .bind((radio_host, radio_port))?
  .run()
  .await
}
