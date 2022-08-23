use std::collections::HashMap;

use awc::{Client, ClientRequest};

use super::model::{OAuthToken, FacebookOAuthToken, GoogleOAuthToken, Provider};

pub struct FacebookCodeExchanger { }

impl FacebookCodeExchanger {

    pub async fn exchange(client: &Client, code: &String, info: HashMap<&str, &str>) -> Result<OAuthToken, String> {
        let url = match Self::build_url(code, info) {
            Some(url) => url,
            None => return Err(format!("URL can't be built"))
        };
        let mut response = client.get(url)
            .send()
            .await
            .map_err(|_| format!("Unknown error"))?;

        let json = response.json::<FacebookOAuthToken>().await;
        let token = match json {
            Ok(token) => token.access_token,
            Err(err) => return Err(format!("Decoding error: {}", err))
        };

        Ok(OAuthToken {
            token,
            provider: Provider::Facebook,
        })        
    }
}

impl FacebookCodeExchanger {

    fn build_url(code: &String, info: HashMap<&str, &str>) -> Option<String> {
        let client_id = match info.get("client_id") {
            Some(client_id) => client_id,
            None => {
                println!("'client_id' key is not found");
                return None
            },
        };
        let client_secret = match info.get("client_secret") {
            Some(client_secret) => client_secret,
            None => {
                println!("'client_secret' key is not found");
                return None
            },
        };
        let redirect_uri = match info.get("redirect_uri") {
            Some(redirect_uri) => redirect_uri,
            None => {
                println!("'redirect_uri' key is not found");
                return None
            },
        };
        Some(format!("https://graph.facebook.com/v14.0/oauth/access_token?client_secret={}&code={}&client_id={}&redirect_uri={}", client_secret, code, client_id, redirect_uri))

    }

}


pub struct GoogleCodeExchanger { }

impl GoogleCodeExchanger {

    pub async fn exchange(client: &Client, code: &String, info: HashMap<&str, &str>) -> Result<OAuthToken, String> {
        let body = match Self::build_body(code, info) {
            Some(body) => body,
            None => return Err(format!("Body could not be built"))
        };
        let mut response = Self::build_request(client)
            .send_body(body)
            .await
            .map_err(|_| format!("Unknown error"))?;

        let json = response.json::<GoogleOAuthToken>().await;
        let token = match json {
            Ok(token) => token.id_token,
            Err(err) => return Err(format!("Error while decoding: {}", err))
        };

        Ok(OAuthToken {
            token,
            provider: Provider::Google,
        })        
    }
}

impl GoogleCodeExchanger {

    fn build_request(client: &Client) -> ClientRequest {
        let url = Self::build_url();

        client.post(url)
            .insert_header(("Content-Type", "application/x-www-form-urlencoded"))
    }

    fn build_url() -> String{
        format!("https://oauth2.googleapis.com/token")
    }

    fn build_body(code: &String, info: HashMap<&str, &str>) -> Option<String> {
        let client_id = match info.get("client_id") {
            Some(client_id) => client_id,
            None => {
                println!("'client_id' key is not found");
                return None
            },
        };
        let redirect_uri = match info.get("redirect_uri") {
            Some(redirect_uri) => redirect_uri,
            None => {
                println!("'redirect_uri' key is not found");
                return None
            },
        }; 
        let grant_type = "authorization_code";
        Some(format!("code={}&client_id={}&redirect_uri={}&grant_type={}", code, client_id, redirect_uri, grant_type))
    }

}