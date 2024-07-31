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
    pub room: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub video: Option<String>,
    pub start_time_zulu: Option<String>,
    pub end_time_zulu: Option<String>,
    pub id: String,
    pub session_id: String,
    pub conference_id: String,
    pub start_slot: Option<String>,
    pub start_slot_zulu: Option<String>,
    pub speakers: Vec<Speaker>,
    pub workshop_prerequisites: Option<String>,
    pub register_loc: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Speaker {
    pub name: String,
    pub twitter: Option<String>,
    pub bio: String,
}

