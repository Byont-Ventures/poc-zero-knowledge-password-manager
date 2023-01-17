use actix_web::{ body::BoxBody, http::header::ContentType, get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use serde::{Serialize, Deserialize};
use crate::HashData;
use crate::password_verification::HashDataVar;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePasswordHashRequest {
    wallet_address: String,
    password: String,
}

#[derive(Serialize)]
pub struct CreatePasswordHashResponse (pub String);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyPassword {
    wallet_address: String,
    password: String,
    pub_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyResponse(pub bool);

impl CreatePasswordHashRequest {
    pub fn create(&self) -> String {
        let mut hash_data = HashData::new(&self.wallet_address, &self.password);
        hash_data.calculate_hash().to_string()
    }
}

impl VerifyPassword {
    pub fn verify(&self) -> bool {
        let hash_data = HashData::new_with_hash(&self.wallet_address, &self.password, &self.pub_hash);
        let hash_data_var = HashDataVar::new(hash_data);
        if let Err(_) = HashDataVar::prove_with_zkp(hash_data_var) {
            return false;
        }
        true
    }
}

/// Verifies user's password
#[post("/password/verify")]
pub async fn verify(request: web::Json<VerifyPassword>) -> impl Responder {

    let response = request.verify();

    let response = VerifyResponse(response);

    HttpResponse::Ok()
        .json(response)
}

/// Creates a hash for the given password
#[post("/password/create")]
pub async fn create(request: web::Json<CreatePasswordHashRequest>) -> impl Responder {

    let response = request.create();
    let response = CreatePasswordHashResponse(response);
    HttpResponse::Ok()
        .json(response)
}