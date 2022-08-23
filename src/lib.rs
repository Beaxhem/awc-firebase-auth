pub mod error;
mod model;
pub mod oauth;

use awc::{Client, http::StatusCode};
use oauth::model::{OAuthToken, SignInWithIdpResponse};
use crate::{error::{ErrorContainer, LoginError, RegisterError}, model::{LoginResponse, RegisterResponse, LoginBody}, oauth::model::SignInWithIdpBody};


#[derive(Clone)]
pub struct Firebase {
    base_url: String,
    auth_token: String,
    client: Client,
}

impl Firebase {

    pub fn auth(base_url: String, auth_token: String) -> Firebase {
        let client = Client::builder()
            .add_default_header(("Content-Type", "application/json"))
            .finish();
        Firebase {
            base_url,
            auth_token,
            client: client,
        }        
    }

}

impl Firebase {

    pub async fn login(&self, email: String, password: String) -> Result<LoginResponse, LoginError> {
        let url = self.sign_in_url();
        let body = LoginBody::new(email, password);
        let mut res = self.client.post(url)
            .send_json(&body)
            .await
            .map_err(|_| LoginError::Unknown)?;


        match res.status() {
            StatusCode::OK => {
                res.json::<LoginResponse>().await
                    .and_then(|body| Ok(body))
                    .map_err(|_| LoginError::Unknown)
            },
            _ => {
                match res.json::<ErrorContainer>().await {
                    Ok(error) => Err(error.error.login_error()),
                    Err(_) => Err(LoginError::Unknown)
                }
            }
        }
    }

}

impl Firebase {

    pub async fn register(&self, email: String, password: String) -> Result<RegisterResponse, RegisterError> {
        let url = self.sign_up_url();
        let body = LoginBody::new(email, password);
        let mut res = self.client.post(url)
            .send_json(&body)
            .await
            .map_err(|_| RegisterError::Unknown)?;


        match res.status() {
            StatusCode::OK => {
                res.json::<RegisterResponse>().await
                    .and_then(|body| Ok(body))
                    .map_err(|_| RegisterError::Unknown)
            },
            _ => {
                match res.json::<ErrorContainer>().await {
                    Ok(error) => Err(error.error.register_error()),
                    Err(_) => Err(RegisterError::Unknown)
                }
            }
        }        
    }

}

impl Firebase {

    pub async fn sign_in_with_idp<'a>(&self, request_uri: &'a str, token: &OAuthToken) -> Result<SignInWithIdpResponse, LoginError> {
        let url = self.sign_in_oauth_url();
        let body = SignInWithIdpBody {
            request_uri,
            post_body: token.to_string(),
            return_secure_token: true,
            return_idp_credential: true,
        };

        let mut response = self.client.post(url.as_str())
            .send_json(&body)
            .await
            .map_err(|_| LoginError::Unknown)?;

        match response.status() {
            StatusCode::OK => {
                let body = response.json::<SignInWithIdpResponse>().await.unwrap();
                Ok(body)
            },
            _ => {
                match response.json::<ErrorContainer>().await {
                    Ok(error) => Err(error.error.login_error()),
                    Err(_) => Err(LoginError::Unknown)
                }
            }
        }

    }

}

impl Firebase {

    fn sign_in_oauth_url(&self) -> String {
        format!("{}/accounts:signInWithIdp?key={}", self.base_url, self.auth_token)
    }

    fn sign_in_url(&self) -> String {
        format!("{}/accounts:signInWithPassword?key={}", self.base_url, self.auth_token)
    }

    fn sign_up_url(&self) -> String {
        format!("{}/accounts:signUp?key={}", self.base_url, self.auth_token) 
    }

}

