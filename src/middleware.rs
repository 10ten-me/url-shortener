use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

/*
** Host Guard
*/

pub struct Host(pub String);

impl <'a, 'r> FromRequest<'a, 'r> for Host {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        println!("{:?}", request);
        Outcome::Success(Self(
            String::from(
                format!("https://{}", request.headers().get_one("host").unwrap())
            )
        ))
    }
}

/*
** DB Guard
** Extracted from Diesel doc - https://rocket.rs/v0.4/guide/state/#databases
*/

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_db_pool(database_url: String) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("db pool")
}

pub struct DB(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Self(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DB {
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
