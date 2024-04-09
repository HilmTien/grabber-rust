#[macro_use]
extern crate dotenv_codegen;

use crate::models::MatchResponse;
use anyhow::{anyhow, Result};
use std::env;

mod models;

async fn find_lobby_id(mut mp_id: u32, acronym: &String) -> Result<u32> {
    let get_match_url = get_match_url_builder();
    loop {
        if lobby_is_valid(get_match_url(mp_id), acronym).await? {
            return Ok(mp_id);
        }
        mp_id += 1;
    }
}

async fn lobby_is_valid(url: String, acronym: &String) -> Result<bool> {
    // let resp = reqwest::get(url).await?.text().await?;
    if let Ok(resp) = reqwest::get(url).await?.json::<MatchResponse>().await {
        // println!("{resp:#?}");
        let name = resp.match_.name;
        return Ok(name.to_lowercase().contains(acronym));
    } else {
        return Ok(false);
    }
}

fn get_match_url_builder() -> impl Fn(u32) -> String {
    let key = dotenv!("OSU_V1_API_KEY");
    let get_match_url = move |mp_id: u32| -> String {
        return format!("https://osu.ppy.sh/api/get_match?k={key}&mp={mp_id}");
    };

    return get_match_url;
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(anyhow!("Missing command line argument mp_id"));
    }

    let mp_id: u32 = args[1].parse().unwrap();
    let acronym = &args[2];

    let found_id = find_lobby_id(mp_id, acronym).await?;
    println!("{}", found_id);
    Ok(())
}
