use std::env;
use std::time::Instant;

mod fortnite_api_io;

#[tokio::main]
async fn main() {
    let now = Instant::now();
    dotenvy::dotenv().expect("❌ Failed to read .env file");
    let elapsed = now.elapsed();
    println!("✅ Loaded .env file in {:?}", elapsed);

    let now = Instant::now();
    let fortnite_io_api_key = env::var("FORTNITE_IO_API_KEY").expect("FORTNITE_IO_API_KEY not set");
    let fortnite_api_io = fortnite_api_io::FortniteApiIo::new(fortnite_io_api_key);
    let elapsed = now.elapsed();
    println!("✅ Created FortniteApiIo struct in {:?}", elapsed);

    let now = Instant::now();
    let items = fortnite_api_io
        .get_items("/v2/items/list")
        .await
        .expect("❌ Failed to get items list");
    let elapsed = now.elapsed();
    println!("✅ Got {} items list in {:?}", { &items.len() }, elapsed);
}
