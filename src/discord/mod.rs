use std::env;
use poise::serenity_prelude as serenity;
use utils::next_time_slot::{ find_sessions_for_next_timeslot, make_embed_for_timeslot};
use utils::models::Program;
use utils::{fetch, parse_json, get_program_url};
use chrono::{Datelike, Utc};

use crate::utils;

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
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    println!("{}", response);
    ctx.say(response).await?;
    Ok(())
}

// Simple command from example
#[poise::command(slash_command)]
async fn next(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let time =  &Utc::now().with_year(2023).unwrap();
    let time_string = time.to_rfc3339();
    let program: Program = parse_json(fetch(get_program_url().unwrap().as_str()).await.unwrap().as_str()).unwrap();
    let sessions = find_sessions_for_next_timeslot(&program.sessions, &time_string);
    let embed = make_embed_for_timeslot(&sessions, &time);

    ctx.send(poise::CreateReply::default()
        .embed(embed)
        .ephemeral(true)
    ).await?;    
    
    Ok(())
}

pub async fn get_client() -> serenity::Client{
    let token = get_token().unwrap();
    let intents = serenity::GatewayIntents::non_privileged();
   
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), next()],
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


