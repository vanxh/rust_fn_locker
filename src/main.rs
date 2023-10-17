use std::env;

mod fortnite_api_io;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("❌ Failed to read .env file");
    println!("✅ Loaded .env file");

    let fortnite_io_api_key = env::var("FORTNITE_IO_API_KEY").expect("FORTNITE_IO_API_KEY not set");
    let fortnite_api_io = fortnite_api_io::FortniteApiIo::new(fortnite_io_api_key);
    println!("✅ Created FortniteApiIo struct");

    fortnite_api_io
        .get_items("/v2/items/list")
        .await
        .expect("❌ Failed to get items list");
}
