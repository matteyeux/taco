use crate::Device;
use std::process;
use stybulate::{Cell, Headers, Style, Table};

/// Print device info in a table, nothing funny just
/// here in case someone wants to use this feature some day.
pub async fn info(model: String) {
    let device = match Device::new(&model).await {
        Ok(device) => device,
        Err(err) => {
            println!("{err}");
            process::exit(1);
        }
    };

    let table = Table::new(
        Style::Fancy,
        vec![
            vec![Cell::from("Name"), Cell::from(&device.name)],
            vec![Cell::from("Identifier"), Cell::from(&device.identifier)],
            vec![Cell::from("Boardconfig"), Cell::from(&device.boardconfig)],
            vec![Cell::from("Platform"), Cell::from(&device.platform)],
            vec![Cell::from("CPID"), Cell::Int(device.cpid as i32)],
            vec![Cell::from("BDID"), Cell::Int(device.bdid as i32)],
        ],
        Some(Headers::from(vec!["Type", "Value"])),
    )
    .tabulate();
    println!("{table}");
}
