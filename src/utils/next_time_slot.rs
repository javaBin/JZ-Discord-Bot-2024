use chrono::Datelike;
use crate::utils::models::{Program, Session, Speaker};
use crate::utils::get_datetime_from_string;

fn find_sessions_for_next_timeslot(sessions: &Vec<Session>, time: &String ) -> Vec<String> {
    let time = get_datetime_from_string(time);
    let day = time.weekday().num_days_from_monday();

}

fn find_next_timeslot(sessions: &Vec<Session>, time: &String) -> chrono::DateTime<chrono::Utc> {
    let date_time = get_datetime_from_string(time);
    let mut closest_sessions = sessions.iter()
        .filter(|session| session.length == "45" || session.length == "60")
        .filter(|session| session.start_time_zulu.is_some())
        .map(|session| get_datetime_from_string(&session.start_time_zulu.as_ref().unwrap()))
        .filter(|start_time|  date_time.lt(start_time))
        .collect::<Vec<chrono::DateTime<chrono::Utc>>>();

    closest_sessions.sort();

    closest_sessions[0]
}

