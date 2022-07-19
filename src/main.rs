
#[macro_use]
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel_postgres::models::{Post, NewPost};
use chrono::{NaiveDateTime};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use diesel_postgres::schema::posts;

    let naive_date_time = chrono::Utc::now().naive_utc();
    let new_post = NewPost {
        title: title,
        body: body,
        published_date:&naive_date_time.clone(),
        last_login:&naive_date_time.clone(),
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
use std::io::{stdin, Read};
#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";
fn main() {
    println!("starting app");
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();

    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    let mut body = String::new();

    stdin().read_to_string(&mut body).unwrap();
    let post = create_post(&connection, title, &body);

    println!("\nSaved draft {} with id {}", title, post.id);
}
