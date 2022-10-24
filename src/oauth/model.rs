use serde::{self, Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

use crate::Firebase;

use super::{
    error::CodeExhangeError,
    exchanger::{FacebookCodeExchanger, GoogleCodeExchanger},
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInWithIdpBody<'a> {
    pub request_uri: &'a str,
    pub post_body: String,
    pub return_secure_token: bool,
    pub return_idp_credential: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInWithIdpResponse {
    #[serde(default)]
    pub full_name: Option<String>,
    pub email: String,
    pub local_id: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Clone, Copy)]
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

#[derive(Deserialize, Debug)]
pub struct OAuthToken {
    pub token: String,
    pub provider: Provider,

    #[serde(default)]
    pub nonce: Option<String>,
}

impl Display for OAuthToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.provider {
            Provider::Google => write!(
                f,
                "id_token={}&providerId={}",
                self.token,
                self.provider.provider_id()
            ),
            Provider::Facebook => write!(
                f,
                "access_token={}&providerId={}",
                self.token,
                self.provider.provider_id()
            ),
            Provider::Apple => write!(
                f,
                "id_token={}&providerId={}&nonce={}",
                self.token,
                self.provider.provider_id(),
                self.nonce.clone().unwrap_or_else(|| "".to_owned())
            ),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct OAuthCode {
    pub code: String,
    pub provider: Provider,
}

pub struct OAuthCodeExchanger {
    pub code: OAuthCode,
}

impl OAuthCodeExchanger {
    pub fn new(code: &OAuthCode) -> Self {
        OAuthCodeExchanger { code: code.clone() }
    }
}

impl OAuthCodeExchanger {
    pub async fn exchange_for_access_token(
        &self,
        info: &HashMap<&'static str, &'static str>,
        client: &Firebase,
    ) -> Result<OAuthToken, CodeExhangeError> {
        match self.code.provider {
            Provider::Google => {
                GoogleCodeExchanger::exchange(&client.client, &self.code.code, info).await
            }
            Provider::Facebook => {
                FacebookCodeExchanger::exchange(&client.client, &self.code.code, info).await
            }
            _ => Err(CodeExhangeError::UnsupportedProvider(self.code.provider)),
        }
    }
}
