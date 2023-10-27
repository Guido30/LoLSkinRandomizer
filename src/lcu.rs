use base64::{engine::general_purpose, Engine as _};
use regex::Regex;
use reqwest::{blocking::Client, header};

use std::error::Error as StdError;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::thread::spawn;

use crate::models;

#[derive(Debug, Clone, Default)]
struct PortAndToken(String, String);

#[derive(Debug, Clone, Default)]
pub struct GameClient {
    port: String,
    auth_token: String,
    auth_token_encoded: String,
    client: Client,
}

fn build_wmic() -> Result<PortAndToken, ()> {
    let re_port = Regex::new(r"--app-port=([0-9]+)").unwrap();
    let re_auth_token = Regex::new(r"--remoting-auth-token=([\w-]+)").unwrap();

    let handle = match spawn(|| {
        Command::new("wmic")
            .args([
                "PROCESS",
                "WHERE",
                "name='LeagueClientUx.exe'",
                "GET",
                "commandline",
            ])
            .creation_flags(0x08000000)
            .output()
    })
    .join()
    {
        Ok(res) => res,
        Err(_) => return Err(()),
    };

    let cmd = match handle {
        Ok(res) => res,
        Err(_) => return Err(()),
    };

    let cmd_output_str: &str = match std::str::from_utf8(&cmd.stdout[..]) {
        Ok(res) => res,
        Err(_) => return Err(()),
    };

    let port = match re_port.captures(cmd_output_str) {
        Some(v) => v.get(1).unwrap().as_str().to_string(),
        _ => return Err(()),
    };

    let auth_token = match re_auth_token.captures(cmd_output_str) {
        Some(v) => v.get(1).unwrap().as_str().to_string(),
        _ => return Err(()),
    };

    Ok(PortAndToken(port, auth_token))
}

impl GameClient {
    pub fn new() -> Self {
        let mut client = Self::default();
        client.build_client();

        // Development Debugging
        #[cfg(debug_assertions)]
        println!(
            "Port {}\nAuth {}\nEncoded {}",
            client.port, client.auth_token, client.auth_token_encoded
        );

        client
    }

    pub fn status(&self) -> bool {
        match build_wmic() {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn retry(&mut self) {
        if !self.status() {
            self.build_client();
        }
    }

    fn build_url(&self, path: &str) -> String {
        let mut path = path;
        if path.starts_with("/") {
            path = &path[1..];
        };
        format!("https://127.0.0.1:{}/{}", self.port, path)
    }

    fn build_credentials(&mut self) {
        let (port, auth_token) = match build_wmic() {
            Ok(res) => (res.0, res.1),
            Err(_) => (self.port.clone(), self.auth_token.clone()),
        };

        self.port = port;
        self.auth_token = auth_token;
        self.auth_token_encoded = general_purpose::STANDARD
            .encode(format!("riot:{}", self.auth_token));
    }

    fn build_client(&mut self) {
        self.build_credentials();

        // Build the request headers
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                format!("Basic {}", self.auth_token_encoded).as_str(),
            )
            .unwrap(),
        );

        // Build the request client
        let lcu_client = Client::builder()
            .default_headers(headers)
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        self.client = lcu_client;
    }

    pub fn call_champ_select_v1_pickable_skin_ids(
        &self,
    ) -> Result<Vec<i32>, Box<dyn StdError>> {
        let url = self.build_url("lol-champ-select/v1/pickable-skin-ids");
        let res = self.client.get(url).send()?;
        let res_str = res.text()?;
        let result: Vec<i32> = serde_json::from_str(&res_str)?;
        Ok(result)
    }

    pub fn call_champ_select_v1_current_champion(
        &self,
    ) -> Result<i32, Box<dyn StdError>> {
        let url = self.build_url("lol-champ-select/v1/current-champion");
        let res = self.client.get(url).send()?;
        let res_str = res.text()?;
        let result: i32 = serde_json::from_str(&res_str)?;
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
        selected_skin_id: i32,
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
}
