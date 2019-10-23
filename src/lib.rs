extern crate wasm_bindgen;

extern crate rusqlite;
extern crate serde;

extern crate zip;
extern crate flate2;

use wasm_bindgen::prelude::*;
use std::path::Path;
use rusqlite::{params, Connection, Result, Row};
use rusqlite::types::{Value};
use rusqlite::types::Value::Text;

//#[wasm_bindgen]
//pub fn unzip_file (file_path: String) -> String {
//}

//impl From<rusqlite::Error> for wasm_bindgen::JsValue {
//    fn from(e: &rusqlite::Error) -> wasm_bindgen::JsValue {
//        wasm_bindgen::JsValue::from_Str(e.to_str())
//    }
//}

#[wasm_bindgen]
pub fn query_db (query_string: &str, db_path: String) -> std::result::Result<String, wasm_bindgen::JsValue> {
    let path = Path::new(std::ffi::OsString::from(db_path).as_os_str());
    let db = Connection::open(path);
    let query = match db {
        Ok(db1) => db1.prepare(&*query_string),
        Err(e) => return Err(wasm_bindgen::JsValue::from_str(&e.to_string())),
    };
    let json_iter = match query {
        Ok(query1) => query1.query_map(params![], to_JSON),
        Err(e) => return Err(wasm_bindgen::JsValue::from_str(&e.to_string())),
    };
    let mut ret_string = "{\n".to_string();
    for json in json_iter {
        match json {
            Ok(s) => ret_string + s,
            Err(e) => return Err(wasm_bindgen::JsValue::from_str(&e.to_string())),
        };
    };
    ret_string.push_str("\n}");
    Ok(ret_string)
}

fn to_JSON<'a> (row: &Row) -> Result<String> {
    let row_value: Value = row.get(0)?;
    match row_value {
        Text(json) => Ok(json),
        _ => panic!("column did not contain text"),
    }
}
