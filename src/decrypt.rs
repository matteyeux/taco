use crate::device::Device;
use reqwest;
use reqwest::StatusCode;

use select::document::Document;
use select::predicate::{Class, Name};

use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

/// Make an HTTP request to the iPhone Wiki to find the actual
/// Firmware key page we need to scrap.
pub async fn get_fw_keys_page(model: String, buildid: String) -> Option<String> {
    let url = format!("https://www.theiphonewiki.com/w/index.php?search={buildid}+{model}&title=Special%3ASearch&go=Go");
    let res = reqwest::Client::new().get(url).send().await.unwrap();

    let raw_html = match res.status() {
        StatusCode::OK => res.text().await.unwrap(),
        _ => panic!("Something went wrong"),
    };

    for line in raw_html.split('\n') {
        let search_str = format!("{buildid}_({model})");
        if line.contains(&search_str) {
            for pattern in line.split(" ") {
                if pattern.contains("wiki/") && pattern.contains(&search_str) {
                    let v: Vec<&str> = pattern.split('"').collect();
                    //println!("here : {}", v[1]);
                    let key_page = format!("https://www.theiphonewiki.com{}", v[1].to_string());
                    return Some(key_page);
                }
            }
        }
    }

    None
}

/// Grab keys from The iPhone Wiki web page
async fn grab_keys(url: String, file: &String) -> String {
    // download actual wiki page
    let resp = reqwest::get(url).await.unwrap();
    assert!(resp.status().is_success());

    // load HTML code in the scrapper
    let document = Document::from(resp.text().await.unwrap().as_str());

    let mut target_val = String::new();

    // Find the wiki id of the filename, which is in HTML class keypage-filename.
    for nodes in document.find(Class("keypage-filename")) {
        let id = nodes.attrs().nth(1).unwrap().1;
        let filename = nodes.text();
        if filename.contains(file) {
            target_val = id.to_string();
        }
    }

    let mut keys = HashMap::new();

    for nodes in document.find(Name("code")) {
        // This is the AST for a single element
        // Element {
        //     name: "code",
        //     attrs: [
        //         (
        //             "id",
        //             "keypage-ibec-key",
        //         ),
        //     ],
        //     children: [
        //         Text(
        //             "ceab418e60b03f6fb3dec96008afc7fcdd74dd17643defca6f374423bdbee2bf",
        //         ),
        //     ],
        // }
        //
        // attr is an iterator which contains a slice :
        // ("id", "keypage-ibec-iv"). So we can to grab the second value
        // at index 1.
        let id = nodes.attrs().next().unwrap().1;
        // same for Children, but there is only one value.
        let key = nodes.children().next().unwrap().text();

        if id.contains("kbag") || key.contains("Unknown") {
            continue;
        } else {
            keys.insert(id, key);
        }
    }

    let mut ivkey = String::new();
    let mut iv = String::new();
    let mut real_key = String::new();

    for (key, value) in keys {
        if key.contains(&target_val) {
            if key[key.len() - 3..key.len()].contains("-iv") {
                iv = value;
            } else {
                real_key = value
            }
        }
    }

    ivkey.push_str(&iv);
    ivkey.push_str(&real_key);
    return ivkey;
}

/// Runs img4 system command
/// Make sure you have it in your PATH.
/// At some point I'll use some crate to path ASN.1 and decrypt the file.
fn decrypt_img4(file: String, output: String, ivkey: String) {
    Command::new("img4")
        .args(["-i", &file, &output, &ivkey])
        .output()
        .expect("failed to execute process is img4 in your $PATH ?");
}

/// Main decrypt function.
/// Gets build by version
/// Grabs the fw keys page on The iPhone Wiki
/// Scraps the Wiki page to get the keys for the said device
/// Decrypts the file with xerub's img4 tool
pub async fn decrypt(model: String, ios_version: String, file: String, key: Option<&str>) {
    let ivkey: String;

    if key.is_none() {
        let mut device = match Device::new(&model).await {
            Ok(device) => device,
            Err(err) => {
                println!("{err}");
                return;
            }
        };

        // get build ID to find the fw keys page
        let buildid = match device.get_build_by_version(&ios_version) {
            Some(build) => build,
            None => {
                eprintln!("[e] Could not get buildid for iOS {ios_version}");
                return;
            }
        };

        println!("[i] Grabbing keys for {model}/{buildid}");

        // Get the fw keys page to scrap
        let fw_keys_page = get_fw_keys_page(model, buildid)
            .await
            .expect("Could not get firmware keys page");

        // Get keys for the file to decrypt
        ivkey = grab_keys(fw_keys_page, &file).await;
    } else {
        ivkey = key.unwrap().to_string();
    }

    if ivkey.len() != 96 {
        eprintln!(
            "[e] key size is wrong it should be 96 instead of {}",
            ivkey.len()
        );
        return;
    }

    println!("[x] IV  : {}", &ivkey[..32]);
    println!("[x] Key : {}", &ivkey[32..]);

    if !Path::new(&file).exists() {
        eprintln!("[e] {file} does not exist");
        return;
    }

    // output filename
    let output = file.replace("im4p", "bin");

    println!("[i] Decrypting {file} to {output}");
    decrypt_img4(file, output, ivkey);
}
