/*
This project is a perfect project for new languages to learn the use
    of buffers/vectors/arrays, networking, json parsing and  use of packages
*/

use reqwest;
use std::error::Error;
use tokio;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct PoemData {
    title: String,
    author: String,
    lines: Vec<String>,
    linecount: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let res = reqwest::get("https://poetrydb.org/random")
    .await?
    .text()
    .await?;
    
    let poemdata_serialised: Vec<PoemData> = serde_json::from_str(&res).unwrap();

    println!("{}\nby {}", poemdata_serialised[0].title, poemdata_serialised[0].author);
    for a in &poemdata_serialised[0].lines {
        println!("{}", a);
    }

    Ok(())
}
