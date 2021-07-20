use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::schema::users;
use crate::schema::users::dsl::users as all_users;
use std::time::SystemTime;
// this is to get users from the database
#[derive(Serialize, Deserialize, Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub mobile: String,
    pub email: String,
    pub first_name: String,
    pub given_name: String,
    pub encrypted_password: String,
    pub avatar: String,
    pub locked_at: SystemTime,
    pub current_sign_in_at: SystemTime,
    pub current_sign_in_ip: String,
    pub last_sign_in_at: SystemTime,
    pub last_sign_in_ip: String,
    pub sign_in_count: i32,
}

#[derive(Debug,  Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub mobile: String,
    pub email: String,
    pub first_name: String,
    pub given_name: String,
    pub encrypted_password: String,
    pub avatar: String,
    pub locked_at: SystemTime,
    pub current_sign_in_at: SystemTime,
    pub current_sign_in_ip: String,
    pub last_sign_in_at: SystemTime,
    pub last_sign_in_ip: String,
    pub sign_in_count: i32,
}
 
// decode request data
#[derive(Deserialize)]
pub struct LoginAccount {
    pub account: String,
    pub password: String
}
// this is to insert users to database
// #[table_name = "users"]
#[derive(Serialize, Deserialize)]
pub struct UserForm {
    pub username: String,
    pub mobile: String,
    pub password: String,
    pub sms: String
}

impl User {
    pub fn full_name(&self)-> String{
       let name = format!("{}{}",self.given_name,self.first_name);
       name
    }
    pub fn query(conn: &PgConnection) -> Vec<User> {
        all_users
        .order(users::id.desc())
        .load::<User>(conn)
        .expect("error!")
    }

    pub fn login(conn: &PgConnection, login:&LoginAccount) ->Vec<User>{
        all_users
        .filter(users::username.eq(&login.account))
        .filter(users::encrypted_password.eq(&login.password))
        .load::<User>(conn)
        .expect("error!")
    }
    pub fn find_by_id(conn: &PgConnection, user_id: i32) ->Vec<User>{
        all_users
        .filter(users::id.eq(user_id))
        .load::<User>(conn)
        .expect("error!")
    }
    pub fn create(conn: &PgConnection, user: UserForm) -> User {
        let user_value=NewUser{
            username: user.username,
            first_name: String::from(""),
            given_name: String::from(""),
            mobile: user.mobile,
            email: String::from(""),
            encrypted_password: String::from("password"),
            avatar: String::from(""),
            locked_at: SystemTime::now(),
            current_sign_in_at: SystemTime::now(),
            current_sign_in_ip: String::from("127.0.0.1"),
            last_sign_in_at: SystemTime::now(),
            last_sign_in_ip: String::from("127.0.0.1"),
            sign_in_count: 0
        };
        diesel::insert_into(users::table)
            .values(&user_value)
            .get_result::<User>(conn)
            .expect("创建失败！有同名用户")
    }

    // pub fn get_user_by_id(user: UserData, conn: &PgConnection) -> Vec<User> {
    //     all_users
    //     .filter(users::id.eq(user.id))
    //     .load::<User>(conn)
    //     .expect("error!")
    // }
    
    pub fn update_attributes( conn: &PgConnection, user: User)-> Vec<User>{
        all_users
        .filter(users::username.eq(user.username))
        .load::<User>(conn)
        .expect("error!")
    }

    pub fn delete_by_id( conn: &PgConnection, id:i32 ) -> usize{
        diesel::delete(users::table.filter(users::id.eq(id)))
        .execute(conn)
        .expect("Failed to clean up users")
    }
}