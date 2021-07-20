use diesel;
use crate::schema::*;
use diesel::prelude::*;
use crate::connection::*;
use crate::models::*;

pub fn last_insert_id(connection: &MysqlDatabase) -> i32 {
    let result: Vec<InsertId> = diesel::sql_query("select last_insert_id() as id").load(&connection.0).unwrap();
    result[0].id
 }