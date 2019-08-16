use rocket::response::Redirect;
use crate::middleware::DB;
use crate::model::Url;

/*
** Shortened URL -> Initial URL (301 redirection)
*/
#[get("/<id>")]
pub fn redirect_to_initial(db: DB, id: String) -> Redirect {
    Redirect::to(match Url::find(&db, id.as_str()) {
        Ok(url_dto) => url_dto.initial,
        Err(_) => String::from(crate::NOT_FOUND),
    })
}

fn redirect(url: String) -> String {
    format!("REDIRECT : {}", url)
}
