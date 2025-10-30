use shared_core::crypt;
use std::str::FromStr;

/// User info from authorization
#[derive(Debug, Clone, serde::Deserialize)]
pub struct User {
    pub uuid: uuid::Uuid,
    pub username: String,
}

/// Jwt authorization scheme.
#[derive(poem_openapi::SecurityScheme)]
#[oai(
    ty = "bearer",
    key_in = "header",
    key_name = "Authorization",
    checker = "check_jwt"
)]
pub struct JwtAuthorization(pub User);

async fn check_jwt(
    request: &poem::Request,
    bearer: poem_openapi::auth::Bearer,
) -> poem::Result<User> {
    // Poem's data is not zero cost.
    // If performance is bad we have to switch frameworks again...
    let data: &super::MiddlewareData = request.data().ok_or(poem::Error::from_status(
        poem::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))?;

    // Make sure JWT is valid
    let jwt_decode = data
        .client
        .get_jwt_factory()
        .decode::<crypt::JwtClaimAccess>(&bearer.token)
        .map_err(|_| poem::Error::from_status(poem::http::StatusCode::UNAUTHORIZED))?;

    Ok(User {
        uuid: uuid::fmt::Hyphenated::from_str(&jwt_decode.sub)
            .unwrap()
            .into_uuid(),
        username: jwt_decode.username,
    })
}
