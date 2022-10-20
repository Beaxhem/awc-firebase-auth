pub mod error;
mod model;
pub mod oauth;

use error::AccountError;
pub use model::RegisterResponse;
use std::sync::Arc;

use crate::{
    error::{ErrorContainer, LoginError, RegisterError},
    model::{LoginBody, LoginResponse},
    oauth::model::SignInWithIdpBody,
};
use awc::{http::StatusCode, Client};
use model::FirebaseRequest;
use oauth::model::{OAuthToken, SignInWithIdpResponse};

#[derive(Clone)]
pub struct Firebase {
    base_url: String,
    auth_token: String,
    client: Arc<Client>,
}

impl Firebase {
    pub fn auth(base_url: String, auth_token: String, client: Arc<Client>) -> Firebase {
        Firebase {
            base_url,
            auth_token,
            client,
        }
    }
}

impl Firebase {
    pub async fn login(
        &self,
        email: String,
        password: String,
    ) -> Result<LoginResponse, LoginError> {
        let url = self.sign_in_url();
        let body = LoginBody::new(email, password);
        let mut res = self
            .client
            .post(url)
            .send_json(&body)
            .await
            .map_err(|_| LoginError::Unknown)?;

        match res.status() {
            StatusCode::OK => res
                .json::<LoginResponse>()
                .await
                .map_err(|_| LoginError::Unknown),
            _ => match res.json::<ErrorContainer>().await {
                Ok(error) => Err(error.error.login_error()),
                Err(_) => Err(LoginError::Unknown),
            },
        }
    }
}

impl Firebase {
    pub async fn register(
        &self,
        email: String,
        password: String,
    ) -> Result<RegisterResponse, RegisterError> {
        let url = self.sign_up_url();
        let body = LoginBody::new(email, password);
        let mut res = self
            .client
            .post(url)
            .send_json(&body)
            .await
            .map_err(|_| RegisterError::Unknown)?;

        match res.status() {
            StatusCode::OK => res
                .json::<RegisterResponse>()
                .await
                .map_err(|_| RegisterError::Unknown),
            _ => match res.json::<ErrorContainer>().await {
                Ok(error) => Err(error.error.register_error()),
                Err(_) => Err(RegisterError::Unknown),
            },
        }
    }
}

impl Firebase {
    pub async fn sign_in_with_idp<'a>(
        &self,
        request_uri: &'a str,
        token: &OAuthToken,
    ) -> Result<SignInWithIdpResponse, LoginError> {
        let url = self.sign_in_oauth_url();
        let body = SignInWithIdpBody {
            request_uri,
            post_body: token.to_string(),
            return_secure_token: true,
            return_idp_credential: true,
        };

        let mut response = self
            .client
            .post(url.as_str())
            .send_json(&body)
            .await
            .map_err(|_| LoginError::Unknown)?;

        match response.status() {
            StatusCode::OK => {
                let body = response.json::<SignInWithIdpResponse>().await.unwrap();
                Ok(body)
            }
            _ => match response.json::<ErrorContainer>().await {
                Ok(error) => Err(error.error.login_error()),
                Err(_) => Err(LoginError::Unknown),
            },
        }
    }
}

impl Firebase {
    pub async fn send_verification_email(&self, token: String) -> Result<(), AccountError> {
        let url = self.send_verification_email_url();
        let body = FirebaseRequest {
            request_type: "VERIFY_EMAIL".to_owned(),
            id_token: token,
        };

        let mut response = self
            .client
            .post(url)
            .send_json(&body)
            .await
            .map_err(|_| AccountError::Unknown)?;

        match response.status() {
            StatusCode::OK => Ok(()),
            _ => match response.json::<ErrorContainer>().await {
                Ok(error) => Err(error.error.account_error()),
                Err(_) => Err(AccountError::Unknown),
            },
        }
    }
}

impl Firebase {
    pub async fn delete_account(&self, token: String) -> Result<(), AccountError> {
        let url = self.delete_account_url();
        let body = FirebaseRequest {
            request_type: "DELETE_ACCOUNT".to_owned(),
            id_token: token,
        };

        let mut response = self
            .client
            .post(url)
            .send_json(&body)
            .await
            .map_err(|_| AccountError::Unknown)?;

        match response.status() {
            StatusCode::OK => Ok(()),
            _ => match response.json::<ErrorContainer>().await {
                Ok(error) => Err(error.error.account_error()),
                Err(_) => Err(AccountError::Unknown),
            },
        }
    }
}

impl Firebase {
    fn sign_in_oauth_url(&self) -> String {
        format!(
            "{}/accounts:signInWithIdp?key={}",
            self.base_url, self.auth_token
        )
    }

    fn sign_in_url(&self) -> String {
        format!(
            "{}/accounts:signInWithPassword?key={}",
            self.base_url, self.auth_token
        )
    }

    fn sign_up_url(&self) -> String {
        format!("{}/accounts:signUp?key={}", self.base_url, self.auth_token)
    }

    fn send_verification_email_url(&self) -> String {
        format!(
            "{}/accounts:sendOobCode?key={}",
            self.base_url, self.auth_token
        )
    }

    fn delete_account_url(&self) -> String {
        format!("{}/accounts:delete?key={}", self.base_url, self.auth_token)
    }
}
