use crate::Device;
use partialzip::partzip::PartialZip;
use std::fs::File;
use std::io::prelude::*;
use std::process;

/// Download a single file
pub async fn download(model: String, version: String, file: String) {
    let mut device = match Device::new(&model).await {
        Ok(device) => device,
        Err(err) => {
            println!("{err}");
            process::exit(1);
        }
    };

    let fw_url = match device.get_firmware_url(&version) {
        Some(fw_url) => fw_url,
        None => {
            eprintln!("[e] Could not get firmware URL for iOS {version}");
            return;
        }
    };

    // split the path to get the real filename
    let v: Vec<&str> = file.split('/').collect();
    let outfile = v[v.len() - 1];

    let real_file = pz_find(&fw_url, &outfile).expect("Could not find file in ipsw");

    println!("[i] Downloading {}", outfile);

    pz_download(&fw_url, &real_file, &outfile).unwrap_or_else(|err| {
        eprintln!("{file} : {err}");
    });
}

/// Find a file in remote zip archive using partialzip
fn pz_find(url: &str, filename: &str) -> Option<String> {
    let mut pz = match PartialZip::new(url) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{e}");
            return None;
        }
    };

    for file in pz.list() {
        if file.name.contains(filename) && !file.name.contains("plist") {
            return Some(file.name);
        }
    }
    None
}

/// Download file from remote zip archive using partialzip
fn pz_download(url: &str, filename: &str, outfile: &str) -> Result<(), String> {
    let mut pz = match PartialZip::new(url) {
        Ok(p) => p,
        Err(e) => return Err(format!("{e}")),
    };

    let data = match pz.download(filename) {
        Ok(content) => content,
        Err(e) => return Err(format!("{e}")),
    };

    match File::create(outfile) {
        Ok(mut f) => {
            if let Err(write_error) = f.write_all(&data) {
                eprintln!("{}", write_error);
                process::exit(1);
            }
        }
        Err(e) => return Err(format!("{e}")),
    };

    Ok(())
}
