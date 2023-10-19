mod fortnite_api;
mod fortnite_api_io;
mod fortnite_locker;

use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::time::Instant;
use tokio;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    create_cache_dir().expect("❌ Failed to create cache directory");
    log_elapsed_time("Created cache directory", start);

    let start = Instant::now();
    dotenvy::dotenv().expect("❌ Failed to read .env file");
    log_elapsed_time("Loaded .env file", start);

    let start = Instant::now();
    let fortnite_io_api_key = env::var("FORTNITE_IO_API_KEY").expect("FORTNITE_IO_API_KEY not set");
    let fortnite_api_io = fortnite_api_io::FortniteApiIo::new(fortnite_io_api_key);
    log_elapsed_time("Created FortniteApiIo struct", start);

    let start = Instant::now();
    let mut fortnite_api = fortnite_api::FortniteAPI::new();
    log_elapsed_time("Created FortniteAPI struct", start);

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

    let start = Instant::now();
    let items = get_locker(fortnite_api, fortnite_api_io).await;
    log_elapsed_time(&format!("Got {} owned outfits", &items.len()), start);

    let start = Instant::now();
    fortnite_locker::FortniteLocker::get_item_img(&items[15])
        .await
        .unwrap();
    log_elapsed_time("Generated outfit image", start);
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

async fn get_locker(
    fortnite_api: fortnite_api::FortniteAPI,
    fortnite_api_io: fortnite_api_io::FortniteApiIo,
) -> Vec<fortnite_api_io::models::items::Item> {
    let start = Instant::now();
    let items = match fortnite_api_io.get_items().await {
        Ok(items) => items,
        Err(err) => panic!("❌ Failed to get items list: {:?}", err),
    };
    log_elapsed_time(&format!("Got {} items list", { &items.len() }), start);

    let start = Instant::now();
    let athena_profile = match fortnite_api.get_athena_profile().await {
        Ok(profile) => profile,
        Err(err) => panic!("❌ Failed to get athena profile: {:?}", err),
    };
    log_elapsed_time("Got athena profile", start);

    let start = Instant::now();

    let owned_outfits: HashSet<String> = athena_profile.profile_changes[0]
        .profile
        .items
        .iter()
        .filter_map(|item| {
            if item.1.template_id.starts_with("AthenaCharacter") {
                Some(
                    item.1
                        .template_id
                        .split(":")
                        .nth(1)?
                        .to_string()
                        .to_lowercase(),
                )
            } else {
                None
            }
        })
        .collect();

    let items = items
        .into_iter()
        .filter(|item| owned_outfits.contains(&item.id.to_lowercase()))
        .collect::<Vec<fortnite_api_io::models::items::Item>>();

    log_elapsed_time("Parsed locker", start);

    items
}

fn create_cache_dir() -> std::io::Result<()> {
    let dir_path = Path::new(".rust_fn_locker");

    if !dir_path.exists() {
        fs::create_dir(dir_path)?;
    }

    Ok(())
}

fn get_memory_usage() -> u64 {
    let process = psutil::process::Process::current().unwrap();
    let memory_info = process.memory_info().unwrap();

    let memory_usage = memory_info.rss() / 1024 / 1024;

    memory_usage
}

fn log_elapsed_time(message: &str, start_time: Instant) {
    let elapsed = start_time.elapsed();
    let memory_usage = get_memory_usage();
    println!("✅ {} in {:?} [{} MB]", message, elapsed, memory_usage);
}
