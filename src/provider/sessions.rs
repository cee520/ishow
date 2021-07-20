use crate::models::user::{User, LoginAccount};
use rocket_contrib::json::Json;
use serde_json::Value;

#[post("/login",format = "application/json",data="<login_user>")]
pub fn login(conn: super::DbConn, login_user: Json<LoginAccount>) -> Json<Value> {
    let user = User::login(&conn, &login_user);
    println!("{}",&login_user.password);
    if user.len()>=1 {
        return Json(json!({
            "code": 0,
            "result":{
                "id": user[0].id,
                "token": "12341234",
                "role": "Admin"
            },
            "message":""
        }))
    }
    else {
        return Json(json!({
            "code": 1,
            "result":{},
            "message":"没有该用户或密码不对！"
        }))
    }
}
 
 
// #[POST(/users, format = application/json, data = <new_user>)]
// pub fn create(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
//     Json(json!({
//         status: User::insert_user(new_user.into_inner(), &conn),
//         result: User::get_all_users(&conn).first(),
//     }))
// }
 
 
// #[GET(/users/<id>, format = application/json)]
// pub fn find(conn: DbConn, user_id: Json<user_id>) -> Json<Value> {
//     Json(json!({
//         status: 200,
//         result: User::get_user_by_id(user_id.into_inner(), &conn),
//     }))
// }