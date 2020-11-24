// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::client;
use prettytable::Table;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub enable_two_factor: bool,
    pub two_factor_verified: bool,
    pub tags: Vec<String>,
    pub session_expiry: u16,
    pub privileges: Vec<Privilege>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Privilege {
    pub scope: String,
    pub org_id: String,
    pub role: String,
    pub name: String,
}

pub async fn get(client: &client::HttpClient) -> Result<Info> {
    let r = client.get(&"self".to_string(), &{}).await.text().await;
    let info: Info = serde_json::from_str(&r.unwrap())?;
    Ok(info)
}

pub async fn print(client: &client::HttpClient) {
    let info = get(client).await.unwrap();
    let mut table = Table::new();
    table.set_titles(row!["User Infomation"]);
    table.add_row(row!["email", info.email]);
    table.add_row(row!["first_name", info.first_name]);
    table.add_row(row!["last_name", info.last_name]);
    table.add_row(row!["enable_two_factor", info.enable_two_factor]);
    table.add_row(row!["two_factor_verified", info.two_factor_verified]);
    table.add_row(row!["tags", info.tags.join("\n")]);
    table.add_row(row!["session_expiry", info.session_expiry]);
    table.printstd();

    for (idx, privilege) in info.privileges.iter().enumerate() {
        let mut table = Table::new();
        table.set_titles(row![format!("User Privilege #{}", idx + 1)]);
        table.add_row(row!["scope", privilege.scope]);
        table.add_row(row!["org_id", privilege.org_id]);
        table.add_row(row!["role", privilege.role]);
        table.add_row(row!["name", privilege.name]);
        table.printstd();
    }
}
