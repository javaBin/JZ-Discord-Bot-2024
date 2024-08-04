use chrono::Datelike;
use serenity::all::CreateEmbed;
use crate::utils::models::Session;
use crate::utils::{get_datetime_from_string, get_session_url_prefix};

pub fn find_sessions_for_next_timeslot(sessions: &Vec<Session>, time: &String ) -> Vec<Session> {
    let date_time = get_datetime_from_string(time);
    let timeslots = get_timeslots(&sessions);
    let next_timeslot = match find_next_timeslot(&timeslots, date_time) {
        Some(timeslot) => timeslot.clone(),
        None => return vec![], // Return an empty vector if no next timeslot is found
    };
    
    let timeslot_after_next = match find_next_timeslot(&timeslots, next_timeslot) {
        Some(timeslot) => timeslot.clone(),
        None => return vec![], // Return an empty vector if no timeslot after the next is found
    };
    // List of sessions that are happening in the next timeslot including lightning talks
    sessions.iter()
        .filter(|session| session.start_time_zulu.is_some())
        .filter(|session| get_datetime_from_string(&session.start_slot_zulu.clone().unwrap()).gt(&date_time) 
                && get_datetime_from_string(&session.start_slot_zulu.clone().unwrap()).lt(&timeslot_after_next))
        .map(|session| session.to_owned())
        .collect()
}

fn get_timeslots(sessions: &Vec<Session>) -> Vec<chrono::DateTime<chrono::Utc>> {
    sessions.iter()
        .filter(|session| session.length == "45" || session.length == "60")
        .filter(|session| session.start_time_zulu.is_some())
        .map(|session| get_datetime_from_string(&session.start_time_zulu.as_ref().unwrap()))
        .collect()
}

fn find_next_timeslot(start_times: &Vec<chrono::DateTime<chrono::Utc>>, time: chrono::DateTime<chrono::Utc>) -> Option<&chrono::DateTime<chrono::Utc>> {
    start_times.iter()
        .filter(|start_time| time.lt(start_time))
        .min()}

fn find_previous_timeslot(start_times: &Vec<chrono::DateTime<chrono::Utc>>, time: chrono::DateTime<chrono::Utc>) -> Option<&chrono::DateTime<chrono::Utc>> {
    start_times.iter()
        .filter(|start_time| time.gt(start_time))
        .max()
}

fn format_session_to_string(session: &Session) -> String {

    /*
    String format: 
    Room: <room> <if lightning talk, add "<time> - Lightning Talk">
    Title: <title with hyperlink to abstract>
    Speakers:\n<speaker1>\n<speaker2>\n<speaker3>
     */
    // let formatted_time = session.start_slot.as_ref()
    //    .map(|start_slot| get_datetime_from_string(start_slot)
    //    .format(": %H:%M").to_string())
    //    .unwrap_or_default();
    let formatted_time = "".to_owned();
    let lightning_talk_info = match session.length.as_str() {
        "10" | "20" => format!("{} - Lightning Talk", formatted_time),
        _ => "".to_owned(),
    };
    let room = match &session.room {
        Some(room) => format!("Room: {}{}", room, lightning_talk_info),
        None => "".to_string(),
    };

    let title = format!("[{}]({}{})", session.title, get_session_url_prefix().unwrap_or_else(|| "".to_owned()), session.id);
    
    let speakers = format!("Speakers:\n{}", session.speakers.iter()
        .map(|speaker| format!("{}\n", speaker.name))
        .collect::<Vec<String>>()
        .join(""));

    format!("***{}***\n{}\n{}", room, title, speakers)
}

pub fn make_embed_for_timeslot(sessions: &Vec<Session>, time: &chrono::DateTime<chrono::Utc>) -> CreateEmbed {
    let embed = poise::serenity_prelude::CreateEmbed::default()
        .title(format!("{} {}", time.weekday(), time.format("%H:%M")))
        .description(sessions.iter()
            .map(|session| format_session_to_string(session))
            .collect::<Vec<String>>()
            .join("\n"));

    embed
}

