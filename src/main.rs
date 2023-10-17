use std::env;
use std::time::Instant;

mod fortnite_api;
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

    let now = Instant::now();
    let fortnite_api = fortnite_api::FortniteAPI::new();
    let elapsed = now.elapsed();
    println!("✅ Created FortniteAPI struct in {:?}", elapsed);

    let now = Instant::now();
    fortnite_login(&fortnite_api).await;
    let elapsed = now.elapsed();
    println!("✅ Logged in to Fortnite in {:?}", elapsed);
}

async fn fortnite_login(fortnite_api: &fortnite_api::FortniteAPI) {
    let switch_token = "OThmN2U0MmMyZTNhNGY4NmE3NGViNDNmYmI0MWVkMzk6MGEyNDQ5YTItMDAxYS00NTFlLWFmZWMtM2U4MTI5MDFjNGQ3";

    let client_token = fortnite_api
        .get_auth_token(switch_token, "grant_type=client_credentials")
        .await
        .expect("❌ Failed to get client token");

    let device_authorization = fortnite_api
        .get_device_authorization(&client_token, "prompt=login")
        .await
        .expect("❌ Failed to get device authorization");

    println!(
        "Visit {} and authorize your account",
        &device_authorization.verification_uri_complete,
    );
}
