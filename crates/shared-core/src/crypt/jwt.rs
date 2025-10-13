use crate::GLOBAL_CONFIG_PATH;

use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey, EncodePublicKey};

/// JWT encryption algorithm
const JWT_ALGORITHM: jsonwebtoken::Algorithm = jsonwebtoken::Algorithm::RS256;

/// Name of private key PEM
const RSA_PEM_PRIVATE: &str = "rsa.pem";

/// Number of bits to use for RSA encryption
const RSA_BITS: usize = 256;

lazy_static::lazy_static! {
    /// Default access token expiration time
    static ref DEFAULT_EXPIRATION_TIME_ACCESS_TOKEN: tokio::time::Duration = {
        // 1 hour
        tokio::time::Duration::from_secs(60 * 60)
    };

    /// Default refresh token expiration time
    static ref DEFAULT_EXPIRATION_TIME_REFRESH_TOKEN: tokio::time::Duration = {
        // 14 days
        tokio::time::Duration::from_secs(60 * 24 * 14)
    };
}

/// Claim for access tokens.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
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

/// Claim for refresh tokens.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct JwtClaimRefresh {
    // (issuer): Issuer of the JWT
    pub iss: String,
    // (subject): Subject of the JWT (the user)
    pub sub: String,
    // (expiration time): Time after which the JWT expires
    pub exp: i64,
}

/// Jwt factory
#[derive(Debug)]
pub struct JwtFactory {
    issuer: String,
    key_private: jsonwebtoken::EncodingKey,
    key_public: jsonwebtoken::DecodingKey,
}

impl JwtFactory {
    /// Create a new instance of the JWT factory.
    /// If an existing private/public key is found then it's loaded otherwise a new one is created and saved.
    pub async fn new(issuer: String) -> Result<Self, tokio::io::Error> {
        let rsa_private_path = GLOBAL_CONFIG_PATH.join(RSA_PEM_PRIVATE);

        // Try load private key from file or create it if it doesn't exist yet.
        let private_key = if rsa_private_path.is_file() {
            tracing::info!("found rsa key: {}", &rsa_private_path.display());

            let rsa_bytes = tokio::fs::read(rsa_private_path).await?;
            let rsa_pem = str::from_utf8(&rsa_bytes).map_err(|_| {
                tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, "invalid utf-8 sequence")
            })?;

            rsa::RsaPrivateKey::from_pkcs8_pem(rsa_pem).map_err(|_| {
                tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, "invalid rsa pem")
            })?
        } else {
            tracing::info!("rsa key does not exist - creating new key");

            let mut rng = rand::thread_rng();
            let rsa_private = rsa::RsaPrivateKey::new(&mut rng, RSA_BITS)
                .map_err(|e| tokio::io::Error::other(e.to_string()))?;

            let rsa_pem = rsa_private
                .to_pkcs8_pem(rsa::pkcs8::LineEnding::default())
                .map_err(|e| tokio::io::Error::other(e.to_string()))?;

            tokio::fs::write(&rsa_private_path, rsa_pem.as_bytes()).await?;

            tracing::info!("saved rsa key: {}", &rsa_private_path.display());

            rsa_private
        };

        // Create encoder and decoder from pem
        let rsa_private_pem = private_key
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::default())
            .expect("failed to get private key pem");

        let rsa_public_pem = private_key
            .to_public_key()
            .to_public_key_pem(rsa::pkcs8::LineEnding::default())
            .expect("failed to get public key pem");

        Ok(Self {
            issuer,
            key_private: jsonwebtoken::EncodingKey::from_rsa_pem(rsa_private_pem.as_bytes())
                .map_err(|e| tokio::io::Error::other(e.to_string()))?,
            key_public: jsonwebtoken::DecodingKey::from_rsa_pem(rsa_public_pem.as_bytes())
                .map_err(|e| tokio::io::Error::other(e.to_string()))?,
        })
    }

    /// Encode claims into signed JWT token.
    pub fn encode<T>(&self, claims: T) -> String
    where
        T: serde::ser::Serialize,
    {
        let header = jsonwebtoken::Header::new(JWT_ALGORITHM);

        jsonwebtoken::encode(&header, &claims, &self.key_private)
            .expect("failed to encode jwt claims")
    }

    pub fn decode<T>(&self, token: &str) -> Result<T, jsonwebtoken::errors::Error>
    where
        T: serde::de::DeserializeOwned + Clone,
    {
        let mut validator = jsonwebtoken::Validation::new(JWT_ALGORITHM);

        validator.validate_exp = true;
        validator.set_issuer(self.issuer.as_bytes());

        jsonwebtoken::decode(token, &self.key_public, &validator).map(|x| x.claims)
    }
}
