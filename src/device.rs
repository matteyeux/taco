use reqwest;
use reqwest::{Response, StatusCode};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug, Clone)]
pub struct Device {
    pub name: String,
    pub identifier: String,
    pub firmwares: Vec<Firmware>,
    pub boards: Vec<Board>,
    pub boardconfig: String,
    pub platform: String,
    pub cpid: u32,
    pub bdid: u8,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Firmware {
    pub identifier: String,
    pub version: String,
    pub buildid: String,
    pub sha1sum: String,
    pub md5sum: String,
    pub sha256sum: String,
    pub filesize: usize,
    pub url: String,
    pub releasedate: String,
    pub uploaddate: String,
    pub signed: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Board {
    pub boardconfig: String,
    pub platform: String,
    pub cpid: u32,
    pub bdid: u8,
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
        for firmware in &self.firmwares {
            if ios_version == &firmware.version {
                return Some(firmware.buildid.clone());
            }
        }

        None
    }

    /// Get firmware URL of an iOS version for a device.
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
}
