pub mod utils;
pub mod discord;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();    
    discord::get_token();

    discord::get_client() 
        .await
        .start()
        .await
        .unwrap();
}
