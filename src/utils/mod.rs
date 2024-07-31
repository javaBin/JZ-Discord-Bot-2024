use reqwest::Client;
use reqwest::header::{HeaderMap, USER_AGENT};
pub mod models;
use crate::utils::models::Program;
pub mod program;
pub mod next_time_slot;
use crate::utils::program::program;

pub async fn fetch(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "reqwest".parse().unwrap());

    let response = client.get(url).headers(headers).send().await?;
    let body = response.text().await?;
    Ok(body)
}


pub async fn fetch_and_print() -> Result<(), reqwest::Error> {
    let res = fetch("https://www.rust-lang.org");

    res.await.map(|text| {
        println!("{}", &text[..100]);
    })
}


// Parse JSON into Program struct
pub fn parse_json(json: &str) -> Result<Program, serde_json::Error> {
    serde_json::from_str(json)
}

pub fn fetch_mock_program() -> Program {
    let prog: String = program();
    
    parse_json(&prog).unwrap()
}


pub fn get_sessions_with_speakers(program: &Program) -> String {
    program.sessions.iter()
    .map(|session| {
        let speakers = session.speakers.iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<&str>>()
            .join(", ");
        format!("{} -> {}", session.title, speakers)
    }
    ).collect::<Vec<String>>()
    .join("\n")
}

fn get_datetime_from_string(date: &str) -> chrono::DateTime<chrono::Utc> {
    let date = chrono::DateTime::parse_from_rfc3339(date).unwrap();
    date.with_timezone(&chrono::Utc)
}
