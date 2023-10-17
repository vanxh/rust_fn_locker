use std::env;
use std::time::Instant;
extern crate psutil;

mod fortnite_api;
mod fortnite_api_io;

#[tokio::main]
async fn main() {
    log_memory_usage();

    let start = Instant::now();
    dotenvy::dotenv().expect("❌ Failed to read .env file");
    log_elapsed_time("Loaded .env file", start);

    log_memory_usage();

    let start = Instant::now();
    let fortnite_io_api_key = env::var("FORTNITE_IO_API_KEY").expect("FORTNITE_IO_API_KEY not set");
    let fortnite_api_io = fortnite_api_io::FortniteApiIo::new(fortnite_io_api_key);
    log_elapsed_time("Created FortniteApiIo struct", start);

    log_memory_usage();

    let start = Instant::now();
    let items = match fortnite_api_io.get_items("/v2/items/list").await {
        Ok(items) => items,
        Err(err) => panic!("❌ Failed to get items list: {:?}", err),
    };
    log_elapsed_time(&format!("Got {} items list", { &items.len() }), start);

    log_memory_usage();

    let start = Instant::now();
    let mut fortnite_api = fortnite_api::FortniteAPI::new();
    log_elapsed_time("Created FortniteAPI struct", start);

    log_memory_usage();

    let start = Instant::now();
    if fortnite_api.session.is_none() {
        fortnite_login(&mut fortnite_api).await;
    }
    log_elapsed_time(
        &format!(
            "Logged in to Fortnite as {}",
            fortnite_api.session.as_ref().unwrap().display_name
        ),
        start,
    );

    log_memory_usage();

    let start = Instant::now();
    let athena_profile = match fortnite_api.get_athena_profile().await {
        Ok(profile) => profile,
        Err(err) => panic!("❌ Failed to get athena profile: {:?}", err),
    };
    log_elapsed_time("Got athena profile", start);

    log_memory_usage();

    let owned_outfits = athena_profile.profile_changes[0]
        .profile
        .items
        .iter()
        .filter(|item| item.1.template_id.starts_with("AthenaCharacter"))
        .map(|item| {
            item.1
                .template_id
                .clone()
                .to_lowercase()
                .split(":")
                .collect::<Vec<&str>>()[1]
                .to_string()
        })
        .collect::<Vec<String>>();

    let items = items
        .iter()
        .filter(|item| owned_outfits.contains(&item.id.to_lowercase()))
        .collect::<Vec<&fortnite_api_io::models::items::Item>>();

    log_elapsed_time(&format!("Got {} owned outfits", items.len()), start);

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

fn log_elapsed_time(message: &str, start_time: Instant) {
    let elapsed = start_time.elapsed();
    println!("✅ {} in {:?}", message, elapsed);
}
