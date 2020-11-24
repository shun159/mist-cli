// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use clap::Clap;

#[derive(Clap, Debug)]
pub struct Options {
    /// email
    #[clap(short = 'e')]
    pub email: String,
    /// password
    #[clap(short = 'p')]
    pub password: String,
    /// two-factor authentication code
    #[clap(short)]
    pub otp_code: Option<u32>,
    #[clap(subcommand)]
    pub subcommand: Commands
}

impl Options {
    pub fn get_args() -> Self {
        let arg: Options = Options::parse();
        arg
    }
}

#[derive(Clap, PartialEq, Debug)]
pub enum Commands {
    ListSites(PrintAllOptions),
    PrintInfo,
    ListWxtags(ListWxtagsOptions),
    AddWxtags(AddWxtagsOptions)
}

#[derive(Clap, PartialEq, Debug)]
/// site related options
pub struct PrintAllOptions {
    /// org UUID string
    pub org_id: String,
}

#[derive(Clap, PartialEq, Debug)]
/// wxtags related options
pub struct ListWxtagsOptions {
    /// site UUID string
    pub site_id: String,
}

#[derive(Clap, PartialEq, Debug)]
/// wxtags related options
pub struct AddWxtagsOptions {
    /// site UUID string
    pub site_id: String,
    pub mac: String,
    pub name: String
}
