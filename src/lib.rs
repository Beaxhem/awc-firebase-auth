pub mod error;
mod model;

use awc::{Client, http::StatusCode};
use crate::{error::{ErrorContainer, LoginError, RegisterError}, model::{LoginResponse, RegisterResponse, LoginBody}};

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

    fn sign_in_url(&self) -> String {
        format!("{}/accounts:signInWithPassword?key={}", self.base_url, self.auth_token)
    }

    fn sign_up_url(&self) -> String {
        format!("{}/accounts:signUp?key={}", self.base_url, self.auth_token) 
    }

}
