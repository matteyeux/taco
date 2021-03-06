use reqwest;
use reqwest::{Response, StatusCode};
use serde::Deserialize;
use std::error::Error;

use crate::decrypt;
use select::predicate::Name;

use select::document::Document;
use stybulate::{Cell, Headers, Style, Table};

#[derive(Deserialize, Clone)]
pub struct Device {
    name: String,
    identifier: String,
    firmwares: Vec<Firmware>,
    boardconfig: String,
    platform: String,
    cpid: u32,
    bdid: u8,
}

#[derive(Deserialize, Clone)]
pub struct Firmware {
    version: String,
    buildid: String,
    url: String,
}

/// Download json from ipsw.me and return a Response object
async fn grab_ipsw_me(model: &String) -> Result<Response, Box<dyn Error>> {
    let url = format!("https://api.ipsw.me/v4/device/{model}");
    return Ok(reqwest::get(url).await?);
}

impl Device {
    /// Create a new Device object which will download the json data from ipsw.me
    pub async fn new(model: &String) -> Result<Self, String> {
        let http_response = match grab_ipsw_me(model).await {
            Ok(response) => response,
            Err(_) => return Err("Could not access ipsw.me API".into()),
        };

        match http_response.status() {
            StatusCode::OK => {
                match http_response.json::<Device>().await {
                    Ok(response) => {
                        return Ok(response);
                    }
                    Err(e) => {
                        return Err(format!("Error parsing json : {e}"));
                    }
                };
            }
            StatusCode::NOT_FOUND => {
                return Err(format!("Device {model} not found"));
            }
            other => {
                return Err(format!("Something went wrong : {other}"));
            }
        };
    }

    /// Parse json data to get the actual build ID of an iOS version.
    pub fn get_build_by_version(&mut self, ios_version: &String) -> Option<String> {
        // Check if there is no dot, it's not an iOS version number
        // So maybe a buildid
        if !ios_version.contains(".") {
            return Some(ios_version.to_string());
        }

        for firmware in &self.firmwares {
            if ios_version == &firmware.version {
                return Some(firmware.buildid.clone());
            }
        }

        None
    }

    /// Get firmware URL of an iOS version for a self.
    pub fn get_firmware_url(&mut self, ios_version: &String) -> Option<String> {
        let buildid = match self.get_build_by_version(ios_version) {
            Some(build) => build,
            None => {
                eprintln!("[e] Could not get buildid for iOS {ios_version}");
                return None;
            }
        };

        for firmware in &self.firmwares {
            if buildid == firmware.buildid {
                return Some(firmware.url.clone());
            }
        }

        None
    }

    /// Parse The iPhone Wiki to get the correct URL
    /// ipsw.me cannot get beta URLs.
    /// At some point I'll try to understand what Siguza's pallas.sh does
    /// And I'll rewrite it in Rust.
    pub async fn get_beta_firmware_url(&mut self, buildid: &String) -> Option<String> {
        let fw_keys_page =
            decrypt::get_fw_keys_page(self.identifier.to_string(), buildid.to_string())
                .await
                .expect("Could not get firmware keys page");

        let resp = reqwest::get(fw_keys_page).await.unwrap();
        assert!(resp.status().is_success());

        let document = Document::from(resp.text().await.unwrap().as_str());
        for nodes in document.find(Name("span")) {
            let id = nodes.attrs().next().unwrap().1;
            if id == "keypage-download" {
                for node in nodes.children() {
                    let url = node.attrs().skip(2).next().unwrap().1;
                    return Some(url.to_string());
                }
                break;
            }
        }

        None
    }

    /// Print device info in a table, nothing funny just
    /// here in case someone wants to use this feature some day.
    pub fn print_info(&mut self) {
        let table = Table::new(
            Style::Fancy,
            vec![
                vec![Cell::from("Name"), Cell::from(&self.name)],
                vec![Cell::from("Identifier"), Cell::from(&self.identifier)],
                vec![Cell::from("Boardconfig"), Cell::from(&self.boardconfig)],
                vec![Cell::from("Platform"), Cell::from(&self.platform)],
                vec![Cell::from("CPID"), Cell::Int(self.cpid as i32)],
                vec![Cell::from("BDID"), Cell::Int(self.bdid as i32)],
            ],
            Some(Headers::from(vec!["Type", "Value"])),
        )
        .tabulate();
        println!("{table}");
    }
}
