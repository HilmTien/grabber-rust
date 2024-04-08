#[macro_use]
extern crate dotenv_codegen;

use crate::models::MatchResponse;

mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = dotenv!("OSU_V1_API_KEY");
    let mp_id = "112312066";
    let url = format!("https://osu.ppy.sh/api/get_match?k={key}&mp={mp_id}");
    // let resp = reqwest::get(url).await?.text().await?;
    let resp = reqwest::get(url).await?.json::<MatchResponse>().await?;
    println!("{resp:#?}");
    Ok(())
}
