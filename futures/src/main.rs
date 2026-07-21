use reqwest;
use serde;
use itertools::Itertools;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct PersonInSpace {
    craft: String,
    name: String
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct PeopleInSpace {
    people: Vec<PersonInSpace>,
    number: i32,
    message: String
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct Type {
    english: String,
    chinese: String,
    japanese: String,
    effective: Vec<String>,
    ineffective: Vec<String>,
    no_effect: Vec<String>
}

async fn get_people_in_space() -> PeopleInSpace {

    println!("Requesting people");

    let resp = reqwest::get(
        "http://api.open-notify.org/astros.json")
        .await.unwrap()
        .json::<PeopleInSpace>()
        .await.unwrap();

    println!("people {}", resp.number);
    resp
}

async fn get_types() -> Vec<Type> {
    
    println!("Requesting types");
    
    let resp = reqwest::get(
        "http://www.timestored.com/data/sample/types.json")
        .await.unwrap()
        .json::<Vec<Type>>()
        .await.unwrap();
    
    println!("types {}", resp.len());
    resp
}

#[tokio::main]
async fn main() -> Result <(), Box <dyn std::error::Error>> {

    let astros = tokio::spawn(async{ get_people_in_space().await});
    let types = tokio::spawn(async{ get_types().await});

    println!("");
    println!("Types of creature: {}",
        types.await?.iter()
        .map(|ele| &ele.english).join(", "));

        println!("");
    println!("These people are all in space: {}",
        astros.await?.people.iter()
        .map(|ele| &ele.name).join(", "));

    Ok(())
}