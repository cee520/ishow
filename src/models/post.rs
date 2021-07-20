// use crate::schema::posts;
use diesel;
// use diesel::pg::PgConnection;
use diesel::prelude::*;
// use super::schema::post;
// use super::schema::users::dsl::users as all_users;


#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    // pub created_at: 
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}