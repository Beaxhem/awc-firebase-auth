use std::collections::HashMap;

use awc::{Client, ClientRequest};

use super::{
    error::CodeExhangeError,
    model::{FacebookOAuthToken, GoogleOAuthToken, OAuthToken, Provider},
};

pub struct FacebookCodeExchanger {}

impl FacebookCodeExchanger {
    pub async fn exchange(
        client: &Client,
        code: &String,
        info: &HashMap<&str, &str>,
    ) -> Result<OAuthToken, CodeExhangeError> {
        let url = Self::build_url(code, info)?;

        let mut response = client
            .get(url)
            .send()
            .await
            .map_err(CodeExhangeError::SendRequestError)?;

        let token = response
            .json::<FacebookOAuthToken>()
            .await
            .map_err(CodeExhangeError::DecodingError)?
            .access_token;

        Ok(OAuthToken {
            token,
            provider: Provider::Facebook,
        })
    }
}

impl FacebookCodeExchanger {
    fn build_url(code: &String, info: &HashMap<&str, &str>) -> Result<String, CodeExhangeError> {
        let client_id = match info.get("client_id") {
            Some(client_id) => client_id,
            None => return Err(CodeExhangeError::ParamError("client_id")),
        };
        let client_secret = match info.get("client_secret") {
            Some(client_secret) => client_secret,
            None => return Err(CodeExhangeError::ParamError("client_secret")),
        };
        let redirect_uri = match info.get("redirect_uri") {
            Some(redirect_uri) => redirect_uri,
            None => return Err(CodeExhangeError::ParamError("redirect_uri")),
        };

        Ok(format!("https://graph.facebook.com/v14.0/oauth/access_token?client_secret={}&code={}&client_id={}&redirect_uri={}", client_secret, code, client_id, redirect_uri))
    }
}

pub struct GoogleCodeExchanger {}

impl GoogleCodeExchanger {
    pub async fn exchange(
        client: &Client,
        code: &String,
        info: &HashMap<&str, &str>,
    ) -> Result<OAuthToken, CodeExhangeError> {
        let body = Self::build_body(code, info)?;
        let mut response = Self::build_request(client)
            .send_body(body)
            .await
            .map_err(CodeExhangeError::SendRequestError)?;

        let token = response
            .json::<GoogleOAuthToken>()
            .await
            .map_err(CodeExhangeError::DecodingError)?
            .id_token;

        Ok(OAuthToken {
            token,
            provider: Provider::Google,
        })
    }
}

impl GoogleCodeExchanger {
    fn build_request(client: &Client) -> ClientRequest {
        let url = Self::build_url();

        client
            .post(url)
            .insert_header(("Content-Type", "application/x-www-form-urlencoded"))
    }

    fn build_url() -> String {
        "https://oauth2.googleapis.com/token".to_string()
    }

    fn build_body(code: &String, info: &HashMap<&str, &str>) -> Result<String, CodeExhangeError> {
        let client_id = match info.get("client_id") {
            Some(client_id) => client_id,
            None => return Err(CodeExhangeError::ParamError("client_id")),
        };
        let redirect_uri = match info.get("redirect_uri") {
            Some(redirect_uri) => redirect_uri,
            None => return Err(CodeExhangeError::ParamError("redirect_uri")),
        };
        let grant_type = "authorization_code";
        Ok(format!(
            "code={}&client_id={}&redirect_uri={}&grant_type={}",
            code, client_id, redirect_uri, grant_type
        ))
    }
}
