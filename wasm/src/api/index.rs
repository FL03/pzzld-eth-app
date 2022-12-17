/*
    Appellation: index <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use gloo::net::{Error, http::{Request, Response}};
use wasm_bindgen::prelude::*;

pub fn landing() -> Result<(), JsError> {
    
    Ok(())
}

pub async fn etherscan(path: &str) -> Result<Response, Error> {
    let basepath = "https://api.etherscan.io/api";
    let endpoint = format!("{}/{}", basepath, path);
    let url = endpoint.as_str();
    Ok( Request::get(url).send().await?)
}