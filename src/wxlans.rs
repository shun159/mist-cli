// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::{client, options};
use prettytable::Table;
use serde::{Deserialize, Serialize};
use serde_json::{Value, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientMACTag {
    #[serde(rename = "type")]
    _type: String,
    name: String,
    #[serde(rename = "match")]
    _match: String,
    values: Vec<String>
}

pub async fn add_wxtag(client: &client::HttpClient, options: &options::AddWxtagsOptions) -> Result<()> {
    let url = format!("sites/{}/wxtags", options.site_id);
    let wxtag = ClientMACTag {
        _type: "match".to_string(),
        _match: "client_mac".to_string(),
        name: options.name.clone(),
        values: vec![options.mac.clone()]
    };
    let r = client.post(&url, &wxtag).await.text().await;
    let tag: Value = serde_json::from_str(&r.unwrap()).unwrap();
    let table = Table::new();
    print_wxtag(&tag, table);
    Ok(())
}

pub async fn list_wxtags(client: &client::HttpClient, site_id: &String) -> Result<()>{
    let url = format!("sites/{}/wxtags", site_id);
    let r = client.get(&url, &{}).await.text().await;
    let tags: Value = serde_json::from_str(&r.unwrap()).unwrap();
    for (idx, tag)in tags.as_array().unwrap_or(&vec![]).iter().enumerate() {
        let mut table = Table::new();
        table.set_titles(row![format!("WXTag #{}", idx + 1)]);
        print_wxtag(tag,  table);
    }
    Ok(())
}

fn print_wxtag(tag: &Value, mut table: Table) {
    table.add_row(row!["operator", tag["op"]]);
    table.add_row(row!["wxtag Id", tag["id"]]);
    let mut values = String::new();
    for value in tag["values"].as_array().unwrap_or(&vec![]) {
        values.push_str(value.as_str().unwrap());
        values.push_str(&"\n")
    };
    table.add_row(row!["Values", values]);
    table.add_row(row!["wxtag Name", tag["name"]]);
    table.add_row(row!["For site", tag["for_site"]]);
    table.add_row(row!["Site Id", tag["site_id"]]);
    table.add_row(row!["Org Id", tag["org_id"]]);
    table.add_row(row!["Created At", tag["created_time"]]);
    table.add_row(row!["Modified At", tag["modified_time"]]);
    table.add_row(row!["Type", tag["type"]]);
    table.add_row(row!["Match", tag["match"]]);
    table.add_row(row!["Resource MAC", tag["resource_mac"]]);
    table.add_row(row!["MAC", tag["mac"]]);
    table.printstd();
}
