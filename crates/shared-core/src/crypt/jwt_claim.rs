use std::ops::Add;

lazy_static::lazy_static! {
    /// Default access token expiration time
    static ref DEFAULT_EXPIRATION_TIME_ACCESS_TOKEN: chrono::Duration = {
        chrono::Duration::hours(1)
    };

    /// Default refresh token expiration time
    static ref DEFAULT_EXPIRATION_TIME_REFRESH_TOKEN: chrono::Duration = {
        chrono::Duration::days(14)
    };
}

/// Claim for access tokens.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JwtClaimAccess {
    // (issuer): Issuer of the JWT
    pub iss: String,
    // (subject): Subject of the JWT (the user)
    pub sub: String,
    // (expiration time): Time after which the JWT expires
    pub exp: i64,

    // Preferred e-mail address
    pub email: String,
}

impl JwtClaimAccess {
    pub fn new<A, B>(issuer: A, user_id: uuid::Uuid, email: B) -> Self
    where
        A: ToString,
        B: ToString,
    {
        Self {
            iss: issuer.to_string(),
            sub: user_id.into(),
            exp: chrono::offset::Utc::now()
                .add(*DEFAULT_EXPIRATION_TIME_ACCESS_TOKEN)
                .timestamp(),
            email: email.to_string(),
        }
    }
}

/// Claim for refresh tokens.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JwtClaimRefresh {
    // (issuer): Issuer of the JWT
    pub iss: String,
    // (subject): Subject of the JWT (the user)
    pub sub: String,
    // (expiration time): Time after which the JWT expires
    pub exp: i64,
}

impl JwtClaimRefresh {
    pub fn new<A>(issuer: A, user_id: uuid::Uuid) -> Self
    where
        A: ToString,
    {
        Self {
            iss: issuer.to_string(),
            sub: user_id.into(),
            exp: chrono::offset::Utc::now()
                .add(*DEFAULT_EXPIRATION_TIME_REFRESH_TOKEN)
                .timestamp(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::crypt::{JwtClaimAccess, JwtClaimRefresh, JwtFactory, JwtFactoryMetadata};

    const RSA_PEM_PKCS_8: &str = "-----BEGIN PRIVATE KEY-----
MIIBVAIBADANBgkqhkiG9w0BAQEFAASCAT4wggE6AgEAAkEAl6zz9vR4GZkePHFN
f81yAKtn2+a0X1B2nKyQWUcXopzF/x2awhu0wXMWV6kxRDHSg5BxBHnvaI09VmEO
A0kxiwIDAQABAkBLaJKWmi7H00ekF1THkJX4XT+ypb3RkYiXFnhh2qWWk4OmdwOV
tzA6aK76AJ+W4pYCYhNZk7OWmMV6NcDuelepAiEA31tNYNLLkXU08cw+GtrbvII1
GeuCVitoGuP2mggyJHUCIQCt18P8JIuHP4HpuQfPvi5czb6TDlIbuSOgHhYbyys9
/wIgFp6bdnvCi+ePxhEGFRgm+q9BC2/zUiCxOU/u0GiWE2UCIBGJSXDe8uBCzMUZ
8CrJoX2lF4tYD3pSc8CMKGjHVuZbAiEAoVHy/Z1AeX4LADMJBjVXAZ3L5ueBB2dP
HCC/me2tP9c=
-----END PRIVATE KEY-----";

    struct TestJwt;

    impl JwtFactoryMetadata for TestJwt {
        const ISSUER: &'static str = "test";
    }

    #[tokio::test]
    async fn access() {
        let jwt = JwtFactory::<TestJwt>::from_pem(RSA_PEM_PKCS_8);

        assert!(jwt.is_ok());

        let jwt = jwt.unwrap();
        let uuid = uuid::Uuid::new_v4();

        let access_jwt = jwt.encode(JwtClaimAccess::new(TestJwt::ISSUER, uuid, "hello@mail.com"));
        let access_jwt = jwt.decode::<JwtClaimAccess>(&access_jwt);

        assert!(access_jwt.is_ok());

        let access_jwt = access_jwt.unwrap();

        assert_eq!(access_jwt.iss, TestJwt::ISSUER);
        assert_eq!(access_jwt.sub, uuid.to_string());
        assert_eq!(access_jwt.email, "hello@mail.com");
    }

    #[tokio::test]
    async fn refresh() {
        let jwt = JwtFactory::<TestJwt>::from_pem(RSA_PEM_PKCS_8);

        assert!(jwt.is_ok());

        let jwt = jwt.unwrap();
        let uuid = uuid::Uuid::new_v4();

        let refresh_jwt = jwt.encode(JwtClaimRefresh::new(TestJwt::ISSUER, uuid));
        let refresh_jwt = jwt.decode::<JwtClaimRefresh>(&refresh_jwt);

        assert!(refresh_jwt.is_ok());

        let refresh_jwt = refresh_jwt.unwrap();

        assert_eq!(refresh_jwt.iss, TestJwt::ISSUER);
        assert_eq!(refresh_jwt.sub, uuid.to_string());
    }
}
