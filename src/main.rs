//! # taco
//!
//! `taco` is a work in progress rewrite of
//! [autodecrypt](https://github.com/matteyeux/autodecrypt), a tool written in Python to download a
//! decrypt iOS firmware images.

mod device;
use device::Device;

mod decrypt;
mod download;
mod info;

use clap::{arg, Command};

#[tokio::main]
async fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("decrypt")
                .about("Decrypt firmware image")
                .arg(arg!(<device> "device model"))
                .arg(arg!(<version> "iOS version"))
                .arg(arg!(<file> "firmware image file"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("download")
                .about("Download firmware image")
                .arg(arg!(<device> "device model"))
                .arg(arg!(<version> "iOS version"))
                .arg(arg!(<file> "firmware image file"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("info")
                .about("info about device")
                .arg(arg!(<device> "device model (eg iPhone12,1)"))
                .arg_required_else_help(true),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("decrypt", args)) => {
            download::download(
                args.value_of("device").expect("required").to_string(),
                args.value_of("version").expect("version").to_string(),
                args.value_of("file").expect("required").to_string(),
            )
            .await;
            decrypt::decrypt(
                args.value_of("device").expect("required").to_string(),
                args.value_of("version").expect("required").to_string(),
                args.value_of("file").expect("required").to_string(),
            )
            .await;
        }
        Some(("download", args)) => {
            download::download(
                args.value_of("device").expect("required").to_string(),
                args.value_of("version").expect("version").to_string(),
                args.value_of("file").expect("required").to_string(),
            )
            .await;
        }
        Some(("info", args)) => {
            info::info(args.value_of("device").expect("required").to_string()).await;
        }
        _ => unreachable!(),
    }
}
