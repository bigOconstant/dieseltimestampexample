extern crate diesel;

use diesel::{Queryable,Insertable};
use chrono::{NaiveDateTime};
use super::schema::posts;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub published_date:NaiveDateTime,
    pub last_login:NaiveDateTime,
}


#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub published_date: &'a NaiveDateTime,
    pub last_login:&'a NaiveDateTime,
}