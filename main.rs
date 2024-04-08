#[macro_use]
extern crate dotenv_codegen;

use crate::models::MatchResponse;
use anyhow::{anyhow, Result};
use std::env;

mod models;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(anyhow!("Missing command line argument mp_id"));
    }

    let mut mp_id: u32 = args[1].parse().unwrap();
    let acronym = args[2].clone();
    let key = dotenv!("OSU_V1_API_KEY");

    loop {
        let url = format!("https://osu.ppy.sh/api/get_match?k={key}&mp={mp_id}");
        // let resp = reqwest::get(url).await?.text().await?;
        if let Ok(resp) = reqwest::get(url).await?.json::<MatchResponse>().await {
            // println!("{resp:#?}");
            let name = resp.match_.name;
            println!("{name}, {mp_id}");
            if name.to_lowercase().contains(&acronym) {
                println!("{}", mp_id);
                break;
            }
        }
        mp_id += 1;
    }

    Ok(())
}
