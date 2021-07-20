use super::db::Conn as DbConn;

use std::{io, path::{Path, PathBuf}};
use rocket::response::NamedFile;



pub mod roots {include!{"provider/roots.rs"}}
pub mod users {include!{"provider/users.rs"}}
pub mod sessions {include!{"provider/sessions.rs"}}




// let my_file = NamedFile::open("/public/index.html");
#[get("/<file..>")]
pub fn assets(file: PathBuf) -> io::Result<NamedFile>{
    NamedFile::open(Path::new("public/").join(file))
}