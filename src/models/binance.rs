use std::{fs};
use serde::{Deserialize, Serialize };
// use serde_json::{Result, Value};
use std::collections::HashMap;

const API_KEY:&str ="D6SLsuvoNGbwl7xD0h29TS7OTyryPoDxkaSfmpgMxb78eV3YcHjXXct2kAB5deGa";
const SECRET_KEY:&str="mgJbQI6iqV4WZZ916Em0IotyjJkYHmTuWqtwL9YXVgVDsexqK3Q0jhPpucKzcAsy";

#[derive(Debug,Deserialize)]
pub enum Method{GET, POST}

#[derive(Debug,Deserialize,Serialize)]
pub struct Info{
    _postman_id: String,
    name: String,
    description: String,
    schema: String
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Query{
    key:String,
    value:Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Url{
    raw: String,
    host: Vec<String>,
    path: Vec<String>,
    #[serde(default)]
    query: Vec<Query>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Header{
    r#type: String,
    key: String,
    value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Request{
    method: String,
    header: Vec<Header>,
    url: self::Url,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Interface{
    name: String,
    request: Option<Request>,
    response: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Title{
    name: String,
    item: Vec<Item>,
    description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone )]
#[serde(untagged)]
pub enum Item{
    L{name: String, item: Vec<Item>, description:String},
    T{name: String, item: Vec<Item>},
    I{name: String, request: Option<Request>, response: Vec<String> }
}
impl Item {
    fn name(&self)->&String{
        match &*self{
            Item::L{name, item: _, description: _} => name,
            Item::T{name,item:_} => name,
            Item::I{name, request:_, response:_ } => name,
            // _ => "".to_string()
        }
    }

    fn interface(&self)->Interface{
        match &*self{
            Item::I{name, request, response } => if let 
                Option::Some(data) = request{
                    Interface{ name:name.to_string(), request: 
                        Some(data.clone()), response:response.to_vec()} }
                else {
                    Interface{ name:name.to_string(), request: None, response:response.to_vec()}},
            _ => Interface{ name:"".to_string(), request: None, response:vec!()},
        }
    }
    fn title(&self)->Title{
        match &*self{
            Item::L{name, item , description } => 
                Title{name: name.to_string(), item: item.to_vec(), description:description.to_string()},
            Item::T{name,item} => 
                Title{name: name.to_string(), item: item.to_vec(), description:"".to_string()},
            _ => Title{name:"".to_string(), item:vec!(), description:"".to_string()}
        }
    }    
    fn items(&self)->Vec<Item>{
        match &*self{
            Item::L{name, item, description} => item.to_vec(),
            Item::T{name,item} => item.to_vec(),
            Item::I{name, request, response } => vec!()
            
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]   
pub struct Event{
    listen: String,
    // #[serde(with = "serde_with::json::nested")]
    script: Script
}

#[derive(Debug, Deserialize, Serialize)]   
pub struct Script{
    r#type: String,
    // #[serde(with = "serde_with::json::nested")]
    exec: Vec<String>
}


#[derive(Debug, Deserialize, Serialize)]   
pub struct Api{
    info: Info,
    // #[serde(with = "serde_with::json::nested")]
    item: Vec<Item>,
    event: Vec<Event>
}

pub fn load_api_def(){
    let text=fs::read_to_string("src/config/binance_spot_api_v1.json")
        .expect("Something went wrong reading the file");
    let api:Api = serde_json::from_str(&text).unwrap();
    for i in &api.item {
        println!("{}",i.name())
    }
    // let n=api["item"].len();
    // let mut i=0
    // while i<n {
    //     println!("{:?}",api["item"][i]["name"]);
    //     i =i+1;
    // }
    // println!("{:?}",api["item"][0]["description"]);
    // println!("{:?}",api["item"][0]["item"][0]);
    // println!("{:?}",api.item);
    // api.item.into_iter();
    let t= api.item[3].items();
    println!("{:?}\n",t[0]);
    println!("{:?}\n",t[0].name());
    let i= t[0].items()[0].interface();
    println!("{:?}\n",t[0].items()[0].interface());
    println!("\nInterface\n\n{:?}",i);
    let mut i_1:String="".into();
    let mut i_2:String="".into();
    if let Some(Request{method,header,url})=i.request{
        let u=url.path.iter().map(|x| x.to_owned() + &"/");
        println!("url:{:?}",u);
        println!("{:?}",u.collect::<Vec<String>>());
        println!("{:?}", url.path.iter()
            .fold(String::from(""), |path,x| path+&x+&"/")
        );
    }
    
    println!("name: {}\nrequest:{:?}\nurl:",i_1,i_2)
}

#[tokio::main]
pub async fn http_get()-> Result<(),Box<dyn std::error::Error>>{
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}





