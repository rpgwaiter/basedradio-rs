use actix_web::middleware::Logger;
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use async_mpd::{MpdClient, cmd};
use regex::Regex;
use std::path::{Path, PathBuf};
use std::{env, fs}; // TODO: async fs
use urlencoding::encode;
use icecast_stats::IcecastStatsRoot;

#[derive(serde::Serialize)]
struct ApiResponse {
  song: Song,
  status: RadioStatus,
}

// #[derive(serde::Serialize)]
// struct MoreInfo {
//   game: TitleLangs,
//   links: Option<InfoSites>,
//   notes: Vec<String>
// }
// #[derive(serde::Serialize)]
// struct InfoSites {
//   wikipedia: Option<String>,
//   khinsider: Option<String>
// }


#[derive(serde::Serialize)]
struct Song {
  album: Option<String>,
  artist: Option<String>,
  cover: String,
  file: String,
  download_link: String,
  game: String,
  system: String,
  title: Option<String>,
}

#[derive(serde::Serialize)]
struct RadioStatus {
  elapsed: u64,
  duration: u64,
  listeners: u32
}

// Probably could be named better
struct MetaInfo {
  game: String,
  system: String,
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

fn get_download_link(file: &str) -> String {
  let filehost_url = env::var("RADIO_FILEHOST_URL").unwrap_or("http://localhost:6969".into());
  let encoded = encode(file).into_owned().replace("%2F", "/");
  return format!("{filehost_url}/{encoded}");
}

// Takes the file path from an mpd status
fn get_cover(file: &str) -> String {
  let regex = Regex::new(r"(?i)^cover\.(gif|jpeg|jpg|png|webp)$").unwrap();
  let music_root = env::var("RADIO_MUSIC_DIR").unwrap_or("/Music".into());
  let filehost_url = env::var("RADIO_FILEHOST_URL").unwrap_or("http://localhost:6969".into());
  let song_full_path: PathBuf = Path::new(&music_root).join(file);
  let song_parent = &song_full_path.parent().unwrap();

  let files = fs::read_dir(song_parent).unwrap();

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

        let encoded = encode(path_str).into_owned().replace("%2F", "/");

        return format!("{filehost_url}{encoded}");
      }
    }
  }
  return format!("{filehost_url}/cover.png");
}

// TODO: err handle
async fn get_icecast_info() -> Result<IcecastStatsRoot, reqwest::Error> {
  let icecast_url = env::var("ICECAST_JSON_URL").unwrap_or("https://cast.based.radio/status-json.xsl".into());

  return reqwest::get(icecast_url)
      .await
      .unwrap()
      .json::<IcecastStatsRoot>()
      .await
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
  let current_song = &playlist[status.song.unwrap() as usize];
  let meta = get_meta(&current_song.file);
  let status = mpd.status().await.unwrap();

  let icecast_info = get_icecast_info().await.unwrap();

  HttpResponse::Ok().json(ApiResponse {
    song: Song {
      album: current_song.clone().album,
      artist: current_song.clone().artist,
      cover: get_cover(&current_song.file),
      file: current_song.clone().file,
      download_link: get_download_link(&current_song.file),
      game: meta.game,
      system: meta.system,
      title: current_song.clone().title,
    },
    status: RadioStatus {
      elapsed: status.elapsed.unwrap().as_secs(),
      duration: status.duration.unwrap().as_secs(),
      listeners: icecast_info.icestats.sources_vec().pop().unwrap().listeners().unwrap_or(0)
    },
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
      .service(get_playing_song)
      .service(get_status)
      .service(get_stats)
      .wrap(Logger::default())
  })
  .bind((radio_host, radio_port))?
  .run()
  .await
}
