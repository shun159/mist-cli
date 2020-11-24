// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::client;
use prettytable::Table;
use serde::{Deserialize, Serialize, Deserializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sites(Vec<Site>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Site {
    pub timezone: String,
    pub country_code: String,
    pub address: String,
    #[serde(deserialize_with = "deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latlng: Option<LatLng>,
    #[serde(deserialize_with = "deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(deserialize_with = "deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lng: Option<f64>,
    pub id: String,
    pub name: String,
    pub org_id: String,
    pub created_time: u32,
    pub modified_time: u32,
    #[serde(deserialize_with = "deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rftemplate_id: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secpolicy_id: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarmtemplate_id: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networktemplate_id: Option<String>,
    pub tzoffset: u32
}

fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where D: Deserializer<'de>,
      T: Deserialize<'de>
{
    Ok(Option::deserialize(deserializer)?)
}


#[derive(Debug, Serialize, Deserialize)]
pub struct LatLng {
    lat: f64,
    lng: f64
}

pub async fn print_all(client: &client::HttpClient, org_id: &String) {
    let url = format!("orgs/{}/sites", org_id);
    let r = client.get(&url, &{}).await.text().await;
    let sites: Sites = serde_json::from_str(&r.unwrap()).unwrap();
    for (idx, site) in sites.0.iter().enumerate() {
        let mut table = Table::new();
        table.set_titles(row![format!("Site #{}", idx + 1)]);
        table.add_row(row!["Timezone", site.timezone]);
        table.add_row(row!["Country Code", site.country_code]);
        table.add_row(row!["Address", site.address]);
        table.add_row(row!["Latitude", site.lat.unwrap_or(0.0)]);
        table.add_row(row!["Longitude", site.lng.unwrap_or(0.0)]);
        table.add_row(row!["Site Id", site.id]);
        table.add_row(row!["Site Name", site.name]);
        table.add_row(row!["Org Id", site.org_id]);
        table.add_row(row!["Created At", site.created_time]);
        table.add_row(row!["Modified At", site.modified_time]);
        table.add_row(row!["RF Template", site.rftemplate_id.clone().unwrap_or("".to_string())]);
        table.add_row(row!["Security Policy Id", site.secpolicy_id.clone().unwrap_or("".to_string())]);
        table.add_row(row!["Alarm Template Id", site.alarmtemplate_id.clone().unwrap_or("".to_string())]);
        table.add_row(row!["Network Template Id", site.networktemplate_id.clone().unwrap_or("".to_string())]);
        table.add_row(row!["Timezone Offset", site.tzoffset]);
        table.printstd();
    }
}
