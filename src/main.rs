// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

extern crate clap;
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate prettytable;

mod options;
mod client;
mod sites;
mod info;
mod wxlans;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let client = client::HttpClient::new();
    match options::Options::get_args().subcommand {
        options::Commands::ListSites(print_opt) => {
            client.login().await;
            sites::print_all(&client, &print_opt.org_id).await;
        },
        options::Commands::PrintInfo => {
            client.login().await;
            info::print(&client).await
        },
        options::Commands::ListWxtags(print_opt) => {
            client.login().await;
            wxlans::list_wxtags(&client, &print_opt.site_id).await;
        },
        options::Commands::AddWxtags(create_opt) => {
            client.login().await;
            wxlans::add_wxtag(&client, &create_opt).await;
        }
    }
}
