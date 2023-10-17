use std::env;
use std::time::Instant;
extern crate psutil;

mod fortnite_api;
mod fortnite_api_io;

#[tokio::main]
async fn main() {
    log_memory_usage();

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
    let mut fortnite_api = fortnite_api::FortniteAPI::new();
    let elapsed = now.elapsed();
    println!("✅ Created FortniteAPI struct in {:?}", elapsed);

    let now = Instant::now();
    if fortnite_api.session.is_none() {
        fortnite_login(&mut fortnite_api).await;
    }
    let elapsed = now.elapsed();
    println!(
        "✅ Logged in to Fortnite as {} in {:?}",
        fortnite_api.session.as_ref().unwrap().display_name,
        elapsed
    );

    let now = Instant::now();
    let _athena_profile = fortnite_api
        .get_athena_profile()
        .await
        .expect("❌ Failed to get athena profile");
    let elapsed = now.elapsed();
    println!("✅ Got athena profile in {:?}", elapsed);

    log_memory_usage();
}

async fn fortnite_login(fortnite_api: &mut fortnite_api::FortniteAPI) {
    let switch_token = "OThmN2U0MmMyZTNhNGY4NmE3NGViNDNmYmI0MWVkMzk6MGEyNDQ5YTItMDAxYS00NTFlLWFmZWMtM2U4MTI5MDFjNGQ3";

    let client_token = fortnite_api
        .get_auth_token(&switch_token, "grant_type=client_credentials", false)
        .await
        .expect("❌ Failed to get client token");

    let start = Instant::now();
    let device_authorization = fortnite_api
        .get_device_authorization(&client_token, "prompt=login")
        .await
        .expect("❌ Failed to get device authorization");

    println!(
        "🚀 Visit {} and authorize your account",
        &device_authorization.verification_uri_complete,
    );

    println!("⏳ Waiting for authorization...");
    while start.elapsed().as_secs() < device_authorization.expires_in.into() {
        let body = format!(
            "grant_type=device_code&device_code={}&token_type=eg1",
            &device_authorization.device_code
        );

        let result = fortnite_api
            .get_auth_token(&switch_token, &body.as_str(), true)
            .await;

        if let Ok(access_token) = result {
            if access_token != "" {
                break;
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(
            device_authorization.interval.into(),
        ))
        .await;
    }
}

fn log_memory_usage() {
    let process = psutil::process::Process::current().unwrap();
    let memory_info = process.memory_info().unwrap();
    println!(
        "🤯 Current process memory usage: {} MB",
        memory_info.rss() / 1024 / 1024
    );
}
