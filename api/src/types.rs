use std::env;

#[derive(serde::Serialize)]
pub struct ApiResponse {
  pub song: Song,
  pub status: RadioStatus,
  pub more_info: MoreInfo,
}

#[derive(serde::Serialize)]
pub struct Song {
  pub album: Option<String>,
  pub artist: Option<String>,
  pub background: Option<String>,
  pub cover: String,
  pub file: String,
  pub download_link: String,
  pub game: String,
  pub system: String,
  pub title: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RadioStatus {
  pub elapsed: u64,
  pub duration: u64,
  pub listeners: u32,
}

// Probably could be named better
pub struct MetaInfo {
  pub game: String,
  pub system: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MoreInfo {
  pub game: Option<TitleLangs>,
  pub links: Option<InfoSites>,
  pub notes: Vec<String>,
}

impl MoreInfo {
  pub fn new() -> MoreInfo {
    let mut notes: Vec<String> = Vec::new();
    let info_email = env::var("MOREINFO_EMAIL").unwrap_or("info@based.radio".into());
    notes.push(format!("We don't have any extra info for this game. If you would like to add some, email {info_email}"));
    MoreInfo {
      game: None,
      links: None,
      notes: notes,
    }
  }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InfoSites {
  pub wikipedia: Option<String>,
  pub khinsider: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TitleLangs {
  pub en: Option<String>,
  pub ja: Option<String>,
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct Updates {
  pub updates: Vec<String>
}
