use rocket_contrib::json::Json;
use serde_derive::{Serialize, Deserialize};
use crate::middleware::DB;
use crate::middleware::Host;
use crate::model::Url;
use crate::msg;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub message: String,
    pub url: Option<String>,
}

impl Output {

    pub fn success(url: String) -> Self {
        Self {
            message: String::from("ok"),
            url: Some(url)
        }
    }

    pub fn error<D>(message: D) -> Self
    where D: std::fmt::Display {
        Self {
            message: format!("{}: {}", msg::ERROR, message),
            url: None
        }
    }
}

/*
** Initial URL -> Shortened URL
*/
#[get("/<url>")]
pub fn build_shortened(host: Host, db: DB, url: String) -> Json<Output> {
    Json(match Url::new_and_save(&db, url.as_str()) {
        Ok(url_dto) => Output::success(format!("{}/{}", host.0, url_dto.id)),
        Err(err) => Output::error(err),
    })
}

