use core::time;

use chrono::{Datelike, Duration};
use serenity::all::{CreateActionRow, CreateButton, CreateEmbed};
use crate::utils::models::Session;
use crate::utils::{get_datetime_from_string, get_session_url_prefix};

use super::get_datetime_from_string_with_local_tz;

pub fn find_sessions_for_next_timeslot(sessions: &Vec<Session>, time: &String ) -> Vec<Session> {
    let date_time = match get_datetime_from_string(time) {
        Some(date_time) => date_time,
        None => return vec![], // Return an empty vector if the date time is not found
    };

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
    let mut result: Vec<Session> = sessions.iter()
        .filter(|session| session.start_time_zulu.is_some())
        .filter(|session| get_datetime_from_string(&session.start_slot_zulu.clone().unwrap()).unwrap() >= next_timeslot )
        .filter(|session| get_datetime_from_string(&session.start_slot_zulu.clone().unwrap()).unwrap() < timeslot_after_next)
        .map(|session| session.clone())
        .collect::<Vec<Session>>();

    result.sort_by(|a: &Session, b: &Session| a.room.cmp(&b.room));    

    result

}

pub fn get_timeslots(sessions: &Vec<Session>) -> Vec<chrono::DateTime<chrono_tz::Tz>> {
    sessions.iter()
        .filter(|session| session.length == "45" || session.length == "60")
        .filter(|session| session.start_time_zulu.is_some())
        .map(|session| get_datetime_from_string(&session.start_time_zulu.as_ref().unwrap()))
        .filter(|time| time.is_some())
        .map(|time| time.unwrap())
        .collect::<std::collections::HashSet<chrono::DateTime<chrono_tz::Tz>>>() // remove duplicates
        .into_iter()
        .collect()
}

pub fn find_next_timeslot(start_times: &Vec<chrono::DateTime<chrono_tz::Tz>>, time: chrono::DateTime<chrono_tz::Tz>) -> Option<&chrono::DateTime<chrono_tz::Tz>> {
    start_times.iter()
        .filter(|start_time| time.lt(start_time))
        .min()
}

pub fn find_previous_timeslot(start_times: &Vec<chrono::DateTime<chrono_tz::Tz>>, time: chrono::DateTime<chrono_tz::Tz>) -> Option<&chrono::DateTime<chrono_tz::Tz>> {
    start_times.iter()
        .filter(|start_time| time.gt(start_time))
        .max()
}

fn format_session_to_string(session: &Session) -> String {
    let formatted_time = session.start_time.as_ref()
        .and_then(|start_time| get_datetime_from_string(start_time))
        .map(|start_time| format!("{}", start_time.format("%H:%M")))
        .unwrap_or_else(|| "".to_owned());

    let lightning_talk_info = match session.length.as_str() {
        "10" | "20" => format!("{} - Lightning Talk", formatted_time),
        _ => "".to_owned(),
    };

    let room = match &session.room {
        Some(room) => format!("{}{}", room, lightning_talk_info),
        None => "".to_string(),
    };

    let title = format!("[{}]({}{})", session.title, get_session_url_prefix().unwrap_or_else(|| "".to_owned()), session.id);
    
    let speakers = format!("Speakers:\n{}", session.speakers.iter()
        .map(|speaker| format!("{}\n", speaker.name))
        .collect::<Vec<String>>()
        .join(""));

    format!("***{}***\n{}\n{}", room, title, speakers)
}

pub fn make_embed_for_timeslot(sessions: &Vec<Session>, current_time: &chrono::DateTime<chrono_tz::Tz>) -> CreateEmbed {
    let sessions_start_time = sessions.iter()
        .filter(|session| session.start_time_zulu.is_some())
        .map(|session| get_datetime_from_string_with_local_tz(&session.start_time_zulu.clone().unwrap()).unwrap())
        .filter(|time| time.gt(current_time))
        .min()
        .unwrap_or_else(|| current_time.clone());

    let embed = poise::serenity_prelude::CreateEmbed::default()
        .title(format!("{} {}", sessions_start_time.weekday(), sessions_start_time.format("%H:%M")))
        .description(sessions.iter()
            .map(|session| format_session_to_string(session))
            .collect::<Vec<String>>()
            .join("\n"));

    embed
}

pub fn make_buttons(next_timeslot: chrono::DateTime<chrono_tz::Tz>, previous_timeslot: chrono::DateTime<chrono_tz::Tz>) -> CreateActionRow {
    let adjusted_next_timeslot = next_timeslot + chrono::Duration::minutes(1);
    let adjusted_previous_timeslot = previous_timeslot - chrono::Duration::minutes(1);
    
    let next_button = poise::serenity_prelude::CreateButton::new(format!("next_{}", adjusted_next_timeslot.to_rfc3339()))
        .label("▶️")
        .style(poise::serenity_prelude::ButtonStyle::Primary);

    let previous_button = poise::serenity_prelude::CreateButton::new(format!("previous_{}", adjusted_previous_timeslot.to_rfc3339()))
        .label("◀️")
        .style(poise::serenity_prelude::ButtonStyle::Primary);


    poise::serenity_prelude::CreateActionRow::Buttons(vec![previous_button, next_button])
}

