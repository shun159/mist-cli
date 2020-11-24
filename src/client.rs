// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::options;
use log::{info, warn};
use reqwest::{Client, Response, StatusCode, header::AUTHORIZATION};
use serde::{Deserialize, Serialize};

const MIST_API_BASE: &'static str = "https://api.mist.com/api/v1";

#[derive(Debug)]
pub struct HttpClient {
    pub client: Client,
    pub baseurl: String,
    pub context: options::Options
}

#[derive(Serialize, Deserialize)]
struct UserCredential {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct UserCredential2FA {
    email: String,
    password: String,
    two_factor: u32
}

impl HttpClient {
    pub fn new() -> Self {
        let baseurl = String::from(MIST_API_BASE);
        let client = Client::builder().cookie_store(true).build().unwrap();
        let options = options::Options::get_args();
        HttpClient { client,  baseurl, context: options }
    }

    pub async fn login(&self) {
        let login_resp = if let Some(code) = self.context.otp_code {
            let auth = UserCredential2FA {
                email: self.context.email.clone(),
                password: self.context.password.clone(),
                two_factor: code
            };
            self.post(&"login".to_string(), &auth).await
        } else {
            let auth = UserCredential {
                email: self.context.email.clone(),
                password: self.context.password.clone()
            };
            self.post(&"login".to_string(), &auth).await
        };

        match login_resp.status() {
            StatusCode::OK =>
                info!("Login successfully"),
            StatusCode::UNAUTHORIZED =>
                warn!("Incorrenct authentication credentials"),
            other => {
                let canonical = other
                    .canonical_reason()
                    .unwrap_or("Unknown response");
                warn!("Unhandled Response: {}", canonical)
            }
        }
    }

    pub async fn get<T: Serialize + ?Sized>(&self, url: &String, body: &T) -> Response {
        let full_url = format!("{}/{}", self.baseurl, url);
        self.client.get(&*full_url)
            .json(body)
            .send()
            .await
            .unwrap()
    }

    pub async fn post<T: Serialize + ?Sized>(&self, url: &String, body: &T) -> Response {
        let full_url = format!("{}/{}", self.baseurl, url);
        self.client.post(&*full_url)
            .json(body)
            .header(AUTHORIZATION, "Token Af062NEYGSEbdiliwu55yYaZtymVQBQJK1sM2W1YBng3hosrZ9w62ksmnXjUya6RNH6yaY83xI1aPAGPSmyNYgCUJXnCkE4c")
            .send()
            .await
            .unwrap()
    }
}
