extern crate serde;

use crate::models::{Data, JsonData, Message};
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde_json::Value as JsonValue;
use std::fs::OpenOptions;
use std::io::{Read, Write};

const FILENAME: &str = "data.json";

pub fn load_file() -> String {
    let mut file = OpenOptions::new().read(true).open(FILENAME).unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    return buff;
}

#[get("/json")]
pub async fn json_get() -> impl Responder {
    let file = load_file();
    let json_value: JsonData = serde_json::from_str(&file).unwrap();
    println!("{:?}", json_value);
    HttpResponse::Ok().json(json_value)
}

#[post("/json")]
pub async fn json_add_name(data: web::Json<Data>) -> impl Responder {
    let mut file = OpenOptions::new().write(true).open(FILENAME).unwrap();
    let file_string = load_file();
    let mut value: JsonData = serde_json::from_str(&file_string).unwrap();
    value
        .names
        .as_array_mut()
        .unwrap()
        .push(JsonValue::from(data.name.to_string()));
    file.write(value.to_string().as_bytes())
        .expect("Couldn't write to file");
    println!("{:?}", value);
    HttpResponse::Ok().json(value)
}

#[delete("/json/clear")]
pub async fn json_clear() -> impl Responder {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILENAME)
        .unwrap();
    let res = r#"
    {"message": "Deleted"}
    "#;
    let value = JsonData {
        names: JsonValue::Array(vec![]),
    };
    file.write_all(value.to_string().as_bytes()).unwrap();
    let res_mes: Message = serde_json::from_str(res).unwrap();
    HttpResponse::Ok().json(res_mes)
}
