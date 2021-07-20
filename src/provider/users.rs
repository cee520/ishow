use crate::models::user::{User, UserForm};
use rocket_contrib::json::Json;
use serde_json::Value;

#[get("/users")]
pub fn index(conn: super::DbConn) -> Json<Value> {
    let users = User::query(&conn);
    Json(json!({
        "code": 0,
        "result": users,
        "message":""
    }))
}
 
 
#[post("/users", format = "application/json", data = "<new_user>")]
pub fn create(conn: super::DbConn, new_user: Json<UserForm>) -> Json<Value> {
    let user = User::create(&conn, new_user.into_inner());
    Json(json!({
        "code": 0,
        "result": user,
        "message":""
    }))
}
 
 
#[get("/users/<id>", format = "application/json")]
pub fn show(conn: super::DbConn, id: i32) -> Json<Value> {
    let user=&User::find_by_id(&conn, id)[0];
    // .push_str(&user.first_name);
    // full_name.push_str(user.first_name);
    Json(json!({
        "code": 0,
        "result": {
            "id":id,
            "username": user.username,
            "full_name": user.full_name(),
            "roles": ["admin"],
            "avatar":"",
            "desc": ""
        },
        "message":""
    }))
}