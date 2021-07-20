#![feature(plugin,  decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use std::env;
// use routes::*;
use std::process::Command;

use binance::api::*;
use binance::account::{Account};
use binance::general::*;
use binance::config::*;
use binance::userstream::*;
use binance::websockets::*;
use std::sync::atomic::{AtomicBool, Ordering};

// use binance::account;

mod db;
mod models;
mod routes;
mod schema{
    include!{"db/schema.rs"}
}
mod channels{pub mod channel;}
use channels::channel::{multiple_streams};

fn rocket() -> rocket::Rocket {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    
    let pool = db::init_pool(database_url);
    rocket::ignite()
    .manage(pool)
    .mount(
        "/",
        routes![routes::roots::index,
            routes::users::index, routes::users::show, routes::users::create, 
            routes::sessions::login],
    )
    .mount("/assets",routes![routes::assets])
    
}
const API_KEY:&str ="D6SLsuvoNGbwl7xD0h29TS7OTyryPoDxkaSfmpgMxb78eV3YcHjXXct2kAB5deGa";
const SECRET_KEY:&str="mgJbQI6iqV4WZZ916Em0IotyjJkYHmTuWqtwL9YXVgVDsexqK3Q0jhPpucKzcAsy";

fn main() {
    // Command::new("sh")
    // .arg("-c")
    // .arg("cd ui && npm start")
    // .spawn()
    // .expect("Failed to start UI Application");
    // rocket().launch();
    // models::binance::load_api_def();
    // models::binance::http_get();
    // let market: Market = Binance::new(None, None);

    // Order book at default depth
    // match market.get_depth("BNBETH") {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {}", e),
    // };
    let api_key = Some(API_KEY.into());
    let secret_key = Some(SECRET_KEY.into());
    let config = Config::default().set_rest_api_endpoint("https:/api.binance.com");
    let general:General= Binance::new_with_config(None, None, &config);
    let account: Account = Binance::new(api_key, secret_key);

    // let ping = account.ping();
    // match ping {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(err) => {
    //         match err.0 {
    //             BinanceLibErrorKind::BinanceError(response) => match response.code {
    //                 -1000_i16 => println!("An unknown error occured while processing the request"),
    //                 _ => println!("Non-catched code {}: {}", response.code, response.msg),
    //             },
    //             BinanceLibErrorKind::Msg(msg) => println!("Binancelib error msg: {}", msg),
    //             _ => println!("Other errors: {}.", err.0),
    //         };
    //     }
    // }

    let result = general.get_server_time();
    match result {
        Ok(answer) => println!("Server Time: {}", answer.server_time),
        Err(e) => println!("Error: {}", e),
    }
    // match account.get_account() {
    //     Ok(answer) => println!("{:?}", answer.balances),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // match account.get_open_orders("WTCETH") {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }
    multiple_streams();
}

