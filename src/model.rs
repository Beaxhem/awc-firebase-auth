use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub kind: String,
    #[serde(rename = "localId")]
    pub local_id: String,
    pub email: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "idToken")]
    pub id_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
    pub registered: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub kind: String,
    #[serde(rename = "localId")]
    pub local_id: String,
    pub email: String,
    #[serde(rename = "idToken")]
    pub id_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginBody {
    pub email: String,
    pub password: String,
    #[serde(rename = "returnSecureToken")]
    pub return_secure_token: bool,
}

impl LoginBody {
    pub fn new(email: String, password: String) -> LoginBody {
        LoginBody {
            email,
            password,
            return_secure_token: true,
        }
    }
}

#[derive(Serialize)]
pub(crate) struct FirebaseRequest {
    #[serde(rename = "requestType")]
    pub request_type: String,
    #[serde(rename = "idToken")]
    pub id_token: String,
}
