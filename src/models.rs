use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CurrentSummonerAccountAndSummonerIds {
    pub account_id: i64,
    pub summoner_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ChampionsCollectionsChampionSkin {
    pub champion_id: i64,
    pub chroma_path: Option<String>,
    pub chromas: Vec<Chroma>,
    pub collection_splash_video_path: Option<String>,
    pub disabled: bool,
    pub emblems: Value,
    pub features_text: Option<String>,
    pub id: i64,
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
#[serde(rename_all = "camelCase", default)]
pub struct ChampSelectChampSelectMySelection {
    pub selected_skin_id: i64,
    pub spell1_id: Option<i64>,
    pub spell2_id: Option<i64>,
    pub ward_skin_id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Chroma {
    pub champion_id: i64,
    pub chroma_path: String,
    pub colors: Vec<String>,
    pub disabled: bool,
    pub id: i64,
    pub last_selected: bool,
    pub name: String,
    pub ownership: Ownership,
    pub still_obtainable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Ownership {
    pub loyalty_reward: bool,
    pub free_to_play_reward: Option<bool>,
    pub owned: bool,
    pub rental: Value,
    pub xbox_g_p_reward: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ChampSelectSession {
    pub id: String,
    pub game_id: i64,
    pub queue_id: i64,
    pub timer: Timer,
    pub chat_details: ChatDetails,
    pub my_team: Vec<Player>,
    pub their_team: Vec<Player>,
    pub trades: Vec<Trade>,
    pub pick_order_swaps: Vec<Trade>,
    pub position_swaps: Vec<Trade>,
    pub actions: Vec<serde_json::Value>,
    pub bans: Bans,
    pub local_player_cell_id: i64,
    pub is_spectating: bool,
    pub allow_skin_selection: bool,
    pub allow_subset_champion_picks: bool,
    pub allow_duplicate_picks: bool,
    pub allow_battle_boost: bool,
    pub boostable_skin_count: i64,
    pub allow_rerolling: bool,
    pub rerolls_remaining: i64,
    pub allow_locked_events: bool,
    pub locked_event_index: i64,
    pub bench_enabled: bool,
    pub bench_champions: Vec<BenchChampion>,
    pub counter: i64,
    pub skip_champion_select: bool,
    pub has_simultaneous_bans: bool,
    pub has_simultaneous_picks: bool,
    pub show_quit_button: bool,
    pub is_legacy_champ_select: bool,
    pub is_custom_game: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Timer {
    pub adjusted_time_left_in_phase: i64,
    pub total_time_in_phase: i64,
    pub phase: String,
    pub is_infinite: bool,
    pub internal_now_in_epoch_ms: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct ChatDetails {
    pub multi_user_chat_id: String,
    pub multi_user_chat_password: String,
    pub muc_jwt_dto: MucJwtDto,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct MucJwtDto {
    pub jwt: String,
    pub channel_claim: String,
    pub domain: String,
    pub target_region: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Player {
    pub cell_id: i64,
    pub champion_id: i64,
    pub selected_skin_id: i64,
    pub ward_skin_id: i64,
    pub spell1_id: i64,
    pub spell2_id: i64,
    pub team: i64,
    pub assigned_position: String,
    pub champion_pick_intent: i64,
    pub player_type: String,
    pub summoner_id: i64,
    pub game_name: String,
    pub tag_line: String,
    pub puuid: String,
    pub is_humanoid: bool,
    pub name_visibility_type: String,
    pub player_alias: String,
    pub obfuscated_summoner_id: i64,
    pub obfuscated_puuid: String,
    pub internal_name: String,
    pub pick_mode: i64,
    pub pick_turn: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Trade {
    pub id: i64,
    pub cell_id: i64,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Bans {
    pub my_team_bans: Vec<i64>,
    pub their_team_bans: Vec<i64>,
    pub num_bans: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct BenchChampion {
    pub champion_id: i64,
    pub is_priority: bool,
}
