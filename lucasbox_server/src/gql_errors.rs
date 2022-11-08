use async_graphql::{Error, ErrorExtensionValues};
use strum_macros::{AsRefStr};

#[derive(Copy, Clone, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    Unauthorized,
    InvalidRefreshToken,
}

pub fn make_gql_error(message: impl Into<String>, code: ErrorCode) -> Error {
    let mut ext = ErrorExtensionValues::default();
    // Set the code extension field to emulate what apollo server does
    ext.set("code", code.as_ref());

    let mut error = Error::new(message);
    error.extensions = Some(ext);

    error
}

#[inline(always)]
pub fn make_unauthorized_error() -> Error {
    make_gql_error("Unauthorized", ErrorCode::Unauthorized)
}
