use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentSummonerAccountAndSummonerIds {
    pub account_id: i64,
    pub summoner_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionsCollectionsChampionSkin {
    pub champion_id: i32,
    pub chroma_path: Option<String>,
    pub chromas: Vec<Chroma>,
    pub collection_splash_video_path: Option<String>,
    pub disabled: bool,
    pub emblems: Value,
    pub features_text: Option<String>,
    pub id: i32,
    pub is_base: bool,
    pub last_selected: bool,
    pub load_screen_path: String,
    pub name: String,
    pub ownership: Value,
    pub quest_skin_info: Value,
    pub rarity_gem_path: String,
    pub skin_type: String,
    pub splash_path: String,
    pub splash_video_path: Option<String>,
    pub still_obtainable: bool,
    pub tile_path: String,
    pub uncentered_splash_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectChampSelectMySelection {
    pub selected_skin_id: i32,
    pub spell1_id: Option<i64>,
    pub spell2_id: Option<i64>,
    pub ward_skin_id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chroma {
    pub champion_id: i32,
    pub chroma_path: String,
    pub colors: Vec<String>,
    pub disabled: bool,
    pub id: i32,
    pub last_selected: bool,
    pub name: String,
    pub ownership: Ownership,
    pub still_obtainable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ownership {
    pub loyalty_reward: bool,
    pub free_to_play_reward: Option<bool>,
    pub owned: bool,
    pub rental: Value,
    pub xbox_g_p_reward: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectSession {
    pub actions: Value,
    pub allow_battle_boost: bool,
    pub allow_duplicate_picks: bool,
    pub allow_locked_events: bool,
    pub allow_rerolling: bool,
    pub allow_skin_selection: bool,
    pub bans: Value,
    pub bench_champions: Value,
    pub bench_enabled: bool,
    pub boostable_skin_count: i32,
    pub chat_details: Value,
    pub counter: i64,
    pub entitled_feature_state: Option<Value>,
    pub game_id: i64,
    pub has_simultaneous_bans: bool,
    pub has_simultaneous_picks: bool,
    pub is_custom_game: bool,
    pub is_spectating: bool,
    pub local_player_cell_id: i64,
    pub locked_event_index: i32,
    pub my_team: Vec<MyTeam>,
    pub recovery_counter: i64,
    pub rerolls_remaining: i32,
    pub skip_champion_select: bool,
    pub their_team: Value,
    pub timer: Value,
    pub trades: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyTeam {
    pub assigned_position: String,
    pub cell_id: i64,
    pub champion_id: i32,
    pub champion_pick_intent: i32,
    pub name_visibility_type: String,
    pub obfuscated_puuid: String,
    pub entitled_feature_type: Option<String>,
    pub puuid: String,
    pub selected_skin_id: i32,
    pub spell1_id: i64,
    pub spell2_id: i64,
    pub summoner_id: i64,
    pub team: i32,
    pub ward_skin_id: i64,
}
