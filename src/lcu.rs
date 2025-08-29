use base64::{engine::general_purpose, Engine as _};
use rand::{seq::SliceRandom, thread_rng};
use regex::Regex;
use reqwest::{blocking::Client, header};

use std::error::Error as StdError;
use std::os::windows::process::CommandExt;
use std::process::Command;

use crate::models::{self, Chroma};

type ChromaTextAndColor = (String, u32);

#[derive(Debug, Clone, Default)]
struct PortAndToken {
    port: String,
    auth_token: String,
}

#[derive(Debug, Clone, Default)]
pub struct GameClient {
    port: String,
    auth_token: String,
    auth_token_encoded: String,
    client: Client,
}

fn build_wmic_wmi() -> Result<PortAndToken, Box<dyn StdError>> {
    let re_port = Regex::new(r"--app-port=([0-9]+)")?;
    let re_auth_token = Regex::new(r"--remoting-auth-token=([\w-]+)")?;

    // Try WMIC first (Windows 10)
    let wmic_cmd = Command::new("wmic")
        .args([
            "PROCESS",
            "WHERE",
            "name='LeagueClientUx.exe'",
            "GET",
            "commandline",
        ])
        .creation_flags(0x08000000)
        .output();

    let output_string: String;
    let cmd_output_str: &str = match wmic_cmd {
        Ok(ref out) if out.status.success() => {
            output_string = String::from_utf8_lossy(&out.stdout).to_string();
            &output_string
        }
        _ => {
            // When wmic fails, try PowerShell WMI (Windows 11)
            let wmi_cmd = r#"Get-CimInstance Win32_Process | Where-Object { $_.Name -eq 'LeagueClientUx.exe' } | Select-Object -ExpandProperty CommandLine"#;
            let wmi_out = Command::new("powershell")
                .args(["-Command", wmi_cmd])
                .creation_flags(0x08000000)
                .output()?;
            output_string =
                String::from_utf8_lossy(&wmi_out.stdout).to_string();
            &output_string
        }
    };

    let port = re_port
        .captures(cmd_output_str)
        .and_then(|v| v.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or("Port not found")?;

    let auth_token = re_auth_token
        .captures(cmd_output_str)
        .and_then(|v| v.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or("Auth token not found")?;

    Ok(PortAndToken { port, auth_token })
}

impl GameClient {
    pub fn new() -> Self {
        let mut client = Self::default();
        let res = client.build_client();
        if res.is_err() {
            println!("LCU client not available");
        }

        // Development Debugging
        #[cfg(debug_assertions)]
        println!(
            "Port {}\nAuth {}\nEncoded {}",
            client.port, client.auth_token, client.auth_token_encoded
        );

        client
    }

    pub fn status(&self) -> bool {
        let url = self.build_url("help");
        self.client.get(url).send().is_ok()
    }

    pub fn retry(&mut self) -> Result<(), Box<dyn StdError>> {
        match self.status() {
            true => Ok(()),
            false => self.build_client(),
        }
    }

    fn build_url(&self, path: &str) -> String {
        let path = path.trim_start_matches("/");
        format!("https://127.0.0.1:{}/{}", self.port, path)
    }

    fn build_client(&mut self) -> Result<(), Box<dyn StdError>> {
        let port_and_token = build_wmic_wmi()?;

        self.port = port_and_token.port;
        self.auth_token = port_and_token.auth_token;
        self.auth_token_encoded = general_purpose::STANDARD
            .encode(format!("riot:{}", self.auth_token));

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                format!("Basic {}", self.auth_token_encoded).as_str(),
            )?,
        );

        let lcu_client = Client::builder()
            .default_headers(headers)
            .danger_accept_invalid_certs(true)
            .build()?;

        self.client = lcu_client;
        Ok(())
    }

    pub fn call_champ_select_v1_pickable_skin_ids(
        &self,
    ) -> Result<Vec<i64>, Box<dyn StdError>> {
        let url = self.build_url("lol-champ-select/v1/pickable-skin-ids");
        let res = self.client.get(url).send()?;
        let res_str = res.text()?;
        let result: Vec<i64> = serde_json::from_str(&res_str)?;
        Ok(result)
    }

    pub fn call_champ_select_v1_current_champion(
        &self,
    ) -> Result<i64, Box<dyn StdError>> {
        let url = self.build_url("lol-champ-select/v1/current-champion");
        let res = self.client.get(url).send()?;
        let res_str = res.text()?;
        let result: i64 = serde_json::from_str(&res_str)?;
        Ok(result)
    }

    pub fn call_summoner_v1_current_summoner_account_and_summoner_ids(
        &self,
    ) -> Result<models::CurrentSummonerAccountAndSummonerIds, Box<dyn StdError>>
    {
        let url = self.build_url(
            "lol-summoner/v1/current-summoner/account-and-summoner-ids",
        );
        let res = self.client.get(url).send()?;
        let res_str = res.text()?;
        let result: models::CurrentSummonerAccountAndSummonerIds =
            serde_json::from_str(&res_str)?;
        Ok(result)
    }

    pub fn call_champions_v1_inventories_summonerid_champions_championid_skins(
        &self,
        summoner_id: i64,
        champion_id: i64,
    ) -> Result<Vec<models::ChampionsCollectionsChampionSkin>, Box<dyn StdError>>
    {
        let url = self.build_url(&format!(
            "lol-champions/v1/inventories/{}/champions/{}/skins",
            summoner_id, champion_id
        ));
        let res = self.client.get(url).send()?;
        let res_str = res.text()?;
        let result: Vec<models::ChampionsCollectionsChampionSkin> =
            serde_json::from_str(&res_str)?;
        Ok(result)
    }

    pub fn call_champ_select_v1_session(
        &self,
    ) -> Result<models::ChampSelectSession, Box<dyn StdError>> {
        let url = self.build_url("lol-champ-select/v1/session");
        let res = self.client.get(url).send()?;
        let res_str = res.text()?;
        let result: models::ChampSelectSession =
            serde_json::from_str(&res_str)?;
        Ok(result)
    }

    pub fn call_champ_select_v1_session_my_selection(
        &self,
        selected_skin_id: i64,
    ) -> Result<(), Box<dyn StdError>> {
        let url = self.build_url("lol-champ-select/v1/session/my-selection");
        let body_str = serde_json::to_string(
            &models::ChampSelectChampSelectMySelection {
                selected_skin_id,
                spell1_id: None,
                spell2_id: None,
                ward_skin_id: None,
            },
        )?;
        self.client.patch(url).body(body_str).send()?;
        Ok(())
    }

    pub fn set_skin(&mut self) -> Result<String, String> {
        if !self.status() {
            return Err("LeagueClient not found!".to_string());
        }

        let summoner_id = self
            .call_summoner_v1_current_summoner_account_and_summoner_ids()
            .map_err(|e| {
                dbg!(e);
                "Failed getting summoner id!".to_string()
            })?;

        let skin_ids =
            self.call_champ_select_v1_pickable_skin_ids().map_err(|e| {
                dbg!(e);
                "Not in champion select!".to_string()
            })?;

        let current_champ =
            self.call_champ_select_v1_current_champion().map_err(|e| {
                dbg!(e);
                "Not in champion select!".to_string()
            })?;

        let champ_skin_ids: Vec<(i64, String)> = self
            .call_champions_v1_inventories_summonerid_champions_championid_skins(
                summoner_id.summoner_id,
                current_champ,
            )
            .map_err(|e| {
                dbg!(e);
                "Champion not picked yet!".to_string()
            })?
            .iter()
            .filter(|skin| skin_ids.contains(&skin.id))
            .map(|skin| (skin.id, skin.name.clone()))
            .collect();

        let Some((skin_id, skin_name)) =
            champ_skin_ids.choose(&mut thread_rng())
        else {
            return Err("No skins available!".to_string());
        };

        self.call_champ_select_v1_session_my_selection(*skin_id)
            .map_err(|e| {
                dbg!(e);
                "Failed changing skin!".to_string()
            })?;

        Ok(skin_name.clone())
    }

    pub fn set_chroma(&self) -> Result<ChromaTextAndColor, String> {
        if !self.status() {
            return Err("LeagueClient not found!".to_string());
        }

        let summoner_id = self
            .call_summoner_v1_current_summoner_account_and_summoner_ids()
            .map_err(|e| {
                dbg!(e);
                "Failed getting summoner id!".to_string()
            })?;

        let champ_select =
            self.call_champ_select_v1_session().map_err(|e| {
                dbg!(e);
                "Skin not picked!".to_string()
            })?;

        let (selected_skin_id, champion_id) = champ_select
            .my_team
            .iter()
            .find(|s| s.summoner_id == summoner_id.summoner_id)
            .map(|s| (s.selected_skin_id, s.champion_id))
            .unwrap_or((0, 0));

        if champion_id == 0 {
            return Err("Skin not picked!".to_string());
        }

        let skin_collection = self
            .call_champions_v1_inventories_summonerid_champions_championid_skins(
                summoner_id.summoner_id,
                champion_id,
            )
            .map_err(|e| {
                dbg!(e);
                "Not in champion select!".to_string()
            })?;

        let current_skin =
            skin_collection.iter().find(|s| s.id == selected_skin_id);

        let mut current_chromas: Vec<&Chroma> = skin_collection
            .iter()
            .find(|skin| {
                skin.chromas.iter().any(|chr| chr.id == selected_skin_id)
            })
            .map(|skin| {
                skin.chromas.iter().filter(|c| c.ownership.owned).collect()
            })
            .unwrap_or_default();

        if current_chromas.is_empty() {
            if let Some(skin) = current_skin {
                current_chromas =
                    skin.chromas.iter().filter(|c| c.ownership.owned).collect();
            }
        }

        let Some(chroma) = current_chromas.choose(&mut thread_rng()) else {
            return Err("No chroma available!".to_string());
        };

        self.call_champ_select_v1_session_my_selection(chroma.id)
            .map_err(|e| {
                dbg!(e);
                "Failed setting chroma".to_string()
            })?;

        u32::from_str_radix(chroma.colors[0].trim_start_matches('#'), 16)
            .map(|color| ("Chroma Randomized!".to_string(), color))
            .map_err(|e| {
                dbg!(e);
                "Invalid color format".to_string()
            })
    }
}
