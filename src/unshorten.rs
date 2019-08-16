use crate::middleware::DB;

/*
** Shortened URL -> Initial URL (301 redirection)
*/
#[get("/<id>")]
pub fn redirect_to_initial(db: DB, id: String) -> String {
    format!("You requested {}. Wonderful!", id)
}
