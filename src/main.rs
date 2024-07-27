pub mod utils;
pub mod discord;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();    
    discord::get_token();
    let program = utils::fetch_mock_program();
    let _sessions = utils::get_sessions_with_speakers(&program);


    // This is blocking, so no point in assigning it to a variable
    discord::get_client() 
        .await
        .start()
        .await
        .unwrap();
}
