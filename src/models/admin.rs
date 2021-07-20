// use crate::schema::*;

// #[derive(Debug)]
// #[derive(Queryable)]

// pub struct Admin {
//     pub id: i32,
//     pub username: String,
//     pub passwd: String,
//     pub salt: String,
//     pub created_at: String,
//     pub updated_at: String,
// }

// #[derive(Debug)]
// #[derive(Queryable)]

// pub struct InsertableAdmin<'a> {
//     pub username: &'a str,
//     pub passwd: &'a str,
//     pub salt: &'a str,
// }

// #[derive(Debug)]
// #[derive(Queryable)]
// #[derive(QueryableByName)]
// #[table_name = "admin_access_token"]
// #[derive(PartialEq)]
// pub struct AdminAccessToken {
//     pub id: i32,
//     pub admin_id: i32,
//     pub access_token: String,
//     pub created_at: String,
//     pub updated_at: String,
// }