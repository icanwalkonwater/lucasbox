use std::time::SystemTime;

use async_graphql::{Context, Error, Object, Result, SimpleObject};
use diesel::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    auth::{generate_password_hash, verify_password_hash},
    extract_connexion,
    GlobalConfig,
    schema_db::{users, user_refresh_tokens},
};
use crate::auth::{generate_jwt_for_user, generate_refresh_token};

/// Main user object
#[derive(Clone, Debug, SimpleObject, Queryable, Identifiable)]
pub struct User {
    pub id: Uuid,
    pub admin: bool,
    pub username: String,
    #[graphql(skip)]
    pub password: String,
    #[graphql(skip)]
    pub updated_at: SystemTime,
    #[graphql(skip)]
    pub created_at: SystemTime,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct AuthTokens {
    access_token: String,
    refresh_token: String,
}

#[derive(Default)]
pub struct UserRootQuery;

#[Object]
impl UserRootQuery {
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let mut _conn = extract_connexion(ctx).await?;
        todo!()
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn register(&self, ctx: &Context<'_>, username: String, password: String, invite_code: Option<String>) -> Result<bool> {
        let mut conn = extract_connexion(ctx).await?;
        let config = ctx.data::<GlobalConfig>().unwrap();

        // Check invite code
        if config.invite_code.is_some() && config.invite_code != invite_code {
            return Err(Error::new("Invalid invite code"));
        }

        // Check if user already exists
        let other_user = users::table
            .filter(users::username.eq(&username))
            .select(users::id)
            .first::<Uuid>(&mut *conn)
            .await
            .optional()?;

        if other_user.is_some() {
            return Err(Error::new("Username already exists"));
        }

        #[derive(Debug, Insertable)]
        #[diesel(table_name = users)]
        struct NewUser {
            username: String,
            password: String,
        }

        // Create user
        let value = NewUser {
            username,
            password: generate_password_hash(&password),
        };

        insert_into(users::table)
            .values(&value)
            .execute(&mut *conn)
            .await?;
        Ok(true)
    }

    async fn login(&self, ctx: &Context<'_>, username: String, password: String) -> Result<AuthTokens> {
        let mut conn = extract_connexion(ctx).await?;

        let user = users::table
            .filter(users::username.eq(username))
            .first::<User>(&mut *conn)
            .await
            .optional()?;

        if user.is_none() || !verify_password_hash(&password, &user.as_ref().unwrap().password) {
            return Err(Error::new("Wrong username or password"));
        }

        let user = user.unwrap();

        let access_token = generate_jwt_for_user(ctx.data().unwrap(), &user);
        let refresh_token = generate_refresh_token();

        #[derive(Debug, Insertable)]
        #[diesel(table_name = user_refresh_tokens)]
        struct NewRefreshToken {
            user_id: Uuid,
            token: String,
        }

        let value = NewRefreshToken {
            user_id: user.id,
            token: refresh_token.clone(),
        };

        insert_into(user_refresh_tokens::table)
            .values(&value)
            .execute(&mut *conn)
            .await?;

        Ok(AuthTokens {
            access_token, refresh_token
        })
    }

    async fn refresh(&self, ctx: &Context<'_>, refresh_token: String) -> Result<AuthTokens> {
        let mut conn = extract_connexion(ctx).await?;

        let user = user_refresh_tokens::table
            .inner_join(users::table)
            .filter(user_refresh_tokens::token.eq(&refresh_token))
            .select(users::all_columns)
            .first::<User>(&mut *conn)
            .await;

        if let Err(diesel::result::Error::NotFound) = user {
            return Err(Error::new("Invalid refresh token"));
        }

        let user = user?;
        let access_token = generate_jwt_for_user(ctx.data().unwrap(), &user);

        Ok(AuthTokens {
            access_token,
            refresh_token,
        })
    }
}
