use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchResponse {
    #[serde(rename = "match")]
    pub match_: MatchInfo,
    pub games: Vec<GameInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchInfo {
    pub match_id: String,
    pub name: String,
    pub start_time: String,
    pub end_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameInfo {
    pub game_id: String,
    pub start_time: String,
    pub end_time: String,
    pub beatmap_id: String,
    pub play_mode: String,
    pub match_type: Option<String>,
    pub scoring_type: String,
    pub team_type: String,
    pub mods: String,
    pub scores: Vec<ScoreInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreInfo {
    pub slot: String,
    pub team: String,
    pub user_id: String,
    pub score: String,
    pub maxcombo: String,
    pub rank: String,
    pub count50: String,
    pub count100: String,
    pub count300: String,
    pub countmiss: String,
    pub countgeki: String,
    pub countkatu: String,
    pub perfect: String,
    pub pass: String,
    pub enabled_mods: Option<String>, // using Option for nullable field
}
