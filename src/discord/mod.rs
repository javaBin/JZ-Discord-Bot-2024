use std::env;
use poise::{serenity_prelude as serenity, FrameworkContext};
use ::serenity::all::CreateInteractionResponseMessage;
use utils::next_time_slot::{ find_sessions_for_next_timeslot, make_embed_for_timeslot, make_buttons, find_next_timeslot, find_previous_timeslot, get_timeslots};
use utils::models::Program;
use utils::{parse_json, get_datetime_from_string};
use chrono::{Datelike, Utc};
use crate::utils::{self, fetch, program};
use crate::utils::program::program;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;




pub fn get_token() -> Option<String> {
    match env::var("DISCORD_TOKEN") {
        Ok(token) => Some(token),
        Err(_) => None,
    }
}

// Simple command from example
#[poise::command(slash_command)]
async fn next(ctx: Context<'_>) -> Result<(), Error> {
    let time =  &Utc::now().with_timezone(&chrono_tz::UTC);
    let time_string = time.to_rfc3339();
    
    let program_url = utils::get_program_url();
    let program_url = match program_url {
        Some(url) => url,
        None => {
            println!("No program url found");
            return Ok(());
        }
    };

    let program = parse_json(&fetch(&program_url).await?);
    println!("program: {:?}", program);
    let program = match program {
        Ok(program) => program,
        Err(_) => {
            println!("Failed to fetch program");
            return Ok(());
        }
    };

    let sessions = find_sessions_for_next_timeslot(&program.sessions, &time_string);
    
    let timeslots = get_timeslots(&program.sessions);
    println!("timeslots: {:?}", timeslots);

    let next_timeslot = match find_next_timeslot(&timeslots, *time) {
        Some(timeslot) => timeslot,
        None => {
            println!("No next timeslot found");
            time
        }
    };
    let previous_timeslot = match find_previous_timeslot(&timeslots, *time) {
        Some(timeslot) => timeslot,
        None => {
            println!("No previous timeslot found");
            time
        }
    };
    
    let embed = make_embed_for_timeslot(&sessions, time);
    let action_row = make_buttons(*next_timeslot, *previous_timeslot);

    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .components(vec![action_row])
        .ephemeral(true)
    ).await?;

    while let Some(mut press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with("next") || press.data.custom_id.starts_with("previous"))
        .timeout(std::time::Duration::from_secs(60))
        .await
    {
        let custom_id = press.data.custom_id.clone();
        let time = custom_id.split("_").last().unwrap();
        let time = get_datetime_from_string(time);
        if let None = time {
            println!("No time found for button");
            continue;
        }

        let time = time.unwrap();
        
        println!("Button id: {}", custom_id);
        println!("Button time: {}", time);

        let next_timeslot = match find_next_timeslot(&timeslots, time) {
        Some(timeslot) => timeslot,
        None => {
            println!("No next timeslot found");
            &time
        }
    };
    let previous_timeslot = match find_previous_timeslot(&timeslots, time) {
        Some(timeslot) => timeslot,
        None => {
            println!("No previous timeslot found");
            &time
        }
    };
    
    let embed = make_embed_for_timeslot(&sessions, &time);
    let action_row = make_buttons(*next_timeslot, *previous_timeslot);

    press.create_response(ctx, serenity::CreateInteractionResponse::UpdateMessage(CreateInteractionResponseMessage::new()
    .embed(embed)
    .components(vec![action_row])))
    .await?;
    }
    
    Ok(())
}

pub async fn get_client() -> serenity::Client{
    let token = get_token().unwrap();
    let intents = serenity::GatewayIntents::non_privileged();
   
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![next()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Failed to create client");

    client
} 


