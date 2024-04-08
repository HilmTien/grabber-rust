#[macro_use]
extern crate dotenv_codegen;

use crate::models::MatchResponse;

mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = dotenv!("OSU_V1_API_KEY");
    let mut mp_id = 0;
    loop {
        let url = format!("https://osu.ppy.sh/api/get_match?k={key}&mp={mp_id}");
        // let resp = reqwest::get(url).await?.text().await?;
        if let Ok(resp) = reqwest::get(url).await?.json::<MatchResponse>().await {
            // println!("{resp:#?}");
            let name = resp.match_.name;
            println!("{name}");
            if name.to_lowercase().contains("acronym") {
                println!("{}", mp_id);
                break;
            }
        }
        mp_id += 1;
    }

    Ok(())
}
