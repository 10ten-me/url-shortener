# URL Shortener

## Endpoints

 * `/shorten/:url_to_shorten` => returns a shortened URL
 * `/:shortened_url` => redirect (301) to initial URL

## Install

### Create POSTGRES database

```
$> sudo -u postgres psql
**
postgres=# CREATE USER shortener WITH ENCRYPTED PASSWORD 'tentenshortener';
postgres=# ALTER USER shortener WITH CREATEDB;
postgres=# \q

$> diesel database setup
$> diesel migration run
```

*Make sure to update .env file according to your psql setup*

### Run

 * `./bin/dev.sh` => run the app for LOCAL
 * `cargo run` => run the app for STAGING
 * `cargo run` => run the app for PROD
