//! Authentication primitives for use elsewhere.

use async_graphql::{Context, Error};

pub use jwt::*;
pub use password::*;

mod password {
    use argon2::{
        password_hash::{rand_core, SaltString},
        Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    };
    use uuid::Uuid;

    pub fn generate_password_hash(password: &str) -> String {
        let hasher = Argon2::default();
        let salt = SaltString::generate(&mut rand_core::OsRng);

        hasher
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string()
    }

    pub fn verify_password_hash(password: &str, hash: &str) -> bool {
        let hasher = Argon2::default();
        let hash = PasswordHash::new(hash).unwrap();

        hasher.verify_password(password.as_bytes(), &hash).is_ok()
    }

    pub fn generate_refresh_token() -> String {
        Uuid::new_v4().to_string()
    }
}

mod jwt {
    use jwt_simple::{claims::Claims, prelude::*};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::{entities::User, GlobalConfig};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct JwtPayload {
        pub user_id: Uuid,
        pub admin: bool,
    }

    pub fn generate_jwt_for_user(config: &GlobalConfig, user: &User) -> String {
        let custom_claims = JwtPayload {
            user_id: user.id,
            admin: user.admin,
        };

        let claims = Claims::with_custom_claims(custom_claims, Duration::from_hours(1));

        config.jwt_key.authenticate(claims).unwrap()
    }

    pub fn verify_jwt_token(config: &GlobalConfig, token: &str) -> Option<JwtPayload> {
        config
            .jwt_key
            .verify_token::<JwtPayload>(token, None)
            .ok()
            .map(|claims| claims.custom)
    }
}

pub(crate) fn make_sure_is_connected<'a>(ctx: &'a Context) -> async_graphql::Result<&'a JwtPayload> {
    if let Some(payload) = ctx.data_opt::<JwtPayload>() {
        Ok(payload)
    } else {
        Err(Error::new("Unauthorized"))
    }
}

pub(crate) fn make_sure_is_admin<'a>(ctx: &'a Context) -> async_graphql::Result<&'a JwtPayload> {
    if let Some(payload) = ctx.data_opt::<JwtPayload>().filter(|auth| auth.admin) {
        Ok(payload)
    } else {
        Err(Error::new("Unauthorized"))
    }
}
