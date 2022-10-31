//! Authentication primitives for use elsewhere.

use std::collections::HashMap;
use std::str::FromStr;

use argon2::{
    password_hash::{rand_core, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use jwt_simple::prelude::*;
use uuid::Uuid;

use crate::{entities::User, GlobalConfig};

pub(crate) fn generate_password_hash(password: &str) -> String {
    let hasher = Argon2::default();
    let salt = SaltString::generate(&mut rand_core::OsRng);

    hasher
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub(crate) fn verify_password_hash(password: &str, hash: &str) -> bool {
    let hasher = Argon2::default();
    let hash = PasswordHash::new(hash).unwrap();

    hasher.verify_password(password.as_bytes(), &hash).is_ok()
}

pub(crate) fn generate_refresh_token() -> String {
    Uuid::new_v4().to_string()
}

pub(crate) fn generate_jwt_for_user(config: &GlobalConfig, user: &User) -> String {
    let custom_claims = HashMap::from([("admin".to_string(), user.admin)]);

    let claims =
        Claims::with_custom_claims(custom_claims, Duration::from_hours(1)).with_subject(user.id);

    config.jwt_key.authenticate(claims).unwrap()
}

pub(crate) fn verify_jwt_token(config: &GlobalConfig, token: &str) -> Option<(Uuid, bool)> {
    if let Ok(claims) = config.jwt_key.verify_token::<HashMap<String, bool>>(token, None) {
        let id = Uuid::from_str(&claims.subject.unwrap()).unwrap();
        let custom_claims = claims.custom;

        let admin = custom_claims["admin"];

        Some((id, admin))
    } else {
        None
    }
}
