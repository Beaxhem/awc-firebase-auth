use std::{fmt::Display, collections::HashMap};
use serde::{Serialize, Deserialize};

use crate::Firebase;

use super::exchanger::{GoogleCodeExchanger, FacebookCodeExchanger};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInWithIdpBody<'a> {
    pub request_uri: &'a str,
    pub post_body: String,
    pub return_secure_token: bool,
    pub return_idp_credential: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInWithIdpResponse {
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub email: String,
    pub local_id: String,
}

#[derive(Debug, Serialize)]
pub enum Provider {
    Facebook,
    Google,
    Apple,
}

impl Display for Provider {
    
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }

}

impl<'de> Deserialize<'de> for Provider {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "facebook.com" => Ok(Provider::Facebook),
            "google.com" => Ok(Provider::Google),
            "apple.com" => Ok(Provider::Apple),
            _ => Err(serde::de::Error::custom(format!("Unknown provider: {}", s))),
        }
    }
}

impl Provider {
    pub fn provider_id(&self) -> String {
        match self {
            Provider::Facebook => "facebook.com".to_string(),
            Provider::Google => "google.com".to_string(),
            Provider::Apple => "apple.com".to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct FacebookOAuthToken {
    pub access_token: String,
}

#[derive(Deserialize, Debug)]
pub struct GoogleOAuthToken {
    pub id_token: String,
}

pub struct OAuthToken {
    pub token: String,
    pub provider: Provider,
}

impl OAuthToken {

    pub fn to_string(&self) -> String {
        match self.provider {
            Provider::Google | Provider::Apple => format!("id_token={}&providerId={}", self.token, self.provider.provider_id()),
            Provider::Facebook => format!("access_token={}&providerId={}", self.token, self.provider.provider_id()),
        }
    }

}


#[derive(Debug, Deserialize)]
pub struct OAuthCode {
    pub code: String,
    pub provider: Provider,
}

impl OAuthCode {

    pub async fn exchange_for_access_token(&self, info: HashMap<&str, &str>, client: &Firebase) -> Result<OAuthToken, String> {
        match self.provider {
            Provider::Google => GoogleCodeExchanger::exchange(&client.client, &self.code, info).await,
            Provider::Facebook => FacebookCodeExchanger::exchange(&client.client, &self.code, info).await,
            _ => Err(format!("Unsupported provider {}", self.provider))
        }
    }

}