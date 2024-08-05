use reqwest::Client;
use reqwest::header::{HeaderMap, USER_AGENT};
use std::env;
pub mod models;
use crate::utils::models::Program;
use chrono_tz::Europe::Oslo;
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

pub fn get_program_url() -> Option<String> {
    match env::var("PROGRAM_URL") {
        Ok(url) => Some(url),
        Err(_) => None,
    }
}


pub fn get_session_url_prefix() -> Option<String> {
    match env::var("SESSION_URL_PREFIX") {
        Ok(url) => Some(url),
        Err(_) => None,
    }
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

fn get_datetime_from_string(date: &str) -> Option<chrono::DateTime<chrono_tz::Tz>> {
    let date = chrono::DateTime::parse_from_rfc3339(date);

    match date {
        Ok(date) => Some(date.with_timezone(&chrono_tz::UTC)),
        Err(_) => None,
    }
}

fn get_datetime_from_string_with_local_tz(date: &str) -> Option<chrono::DateTime<chrono_tz::Tz>> {
    let date = chrono::DateTime::parse_from_rfc3339(date);

    match date {
        Ok(date) => Some(date.with_timezone(&Oslo)),
        Err(_) => None,
    }
}