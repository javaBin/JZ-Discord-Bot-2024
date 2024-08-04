pub mod utils;
pub mod discord;
use dotenv::dotenv;
use chrono::prelude::*;
use utils::next_time_slot::find_sessions_for_next_timeslot;
use utils::{fetch, parse_json};

#[tokio::main]
async fn main() {
    dotenv().ok();    
    discord::get_token();

    //let program = utils::fetch_mock_program();
    // let _sessions = utils::get_sessions_with_speakers(&program);

    // This is blocking, so no point in assigning it to a variable
    discord::get_client() 
        .await
        .start()
        .await
        .unwrap();
}
