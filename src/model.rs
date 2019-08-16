use serde_derive::{Deserialize, Serialize};
use crate::schema::urls;
use diesel;
use diesel::prelude::*;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "urls"]
pub struct Url {
    pub id: String,
    pub initial: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Url {

    pub fn new_and_save(db: &PgConnection, initial: &str) -> QueryResult<Url> {
        diesel::insert_into(urls::table)
            .values(Self {
                id: crate::generator::get_uuid(),
                initial: String::from(initial),
                created_at: chrono::Utc::now().naive_utc(),
            })
            .get_result::<Url>(db)
    }

}
