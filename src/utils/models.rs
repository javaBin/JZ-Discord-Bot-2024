use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Program {
    pub sessions: Vec<Session>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub intended_audience: String,
    pub length: String,
    pub format: String,
    pub language: String,
    #[serde(rename = "abstract")]
    pub abstract_field: String,
    pub title: String,
    pub id: String,
    pub session_id: String,
    pub conference_id: String,
    pub speakers: Vec<Speaker>,
    pub workshop_prerequisites: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Speaker {
    pub name: String,
    pub twitter: Option<String>,
    pub bio: String,
}

