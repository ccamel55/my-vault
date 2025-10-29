use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey, EncodePublicKey};
use std::marker::PhantomData;
use std::path::Path;

/// JWT encryption algorithm
const JWT_ALGORITHM: jsonwebtoken::Algorithm = jsonwebtoken::Algorithm::RS256;

/// Number of bits to use for RSA encryption
const RSA_BITS: usize = 512;

/// Trait for providing specialised info about a Jwt Factor
pub trait JwtFactoryMetadata {
    /// Issuer
    const ISSUER: &'static str;
}

/// Trait for providing claim audience.
/// This is useful for defining specific token types.
pub trait JwtClaimMetadata {
    /// Audience.
    const AUDIENCE: &'static str;
}

/// Jwt factory
#[derive(Debug)]
pub struct JwtFactory<I: JwtFactoryMetadata> {
    key_private: jsonwebtoken::EncodingKey,
    key_public: jsonwebtoken::DecodingKey,
    issuer_type: PhantomData<I>,
}

impl<I: JwtFactoryMetadata> JwtFactory<I> {
    /// Create a new instance of the JWT factory.
    /// If an existing private/public key is found then it's loaded otherwise a new one is created and saved.
    pub async fn new(rsa_private_path: &Path) -> Result<Self, tokio::io::Error> {
        // Try load private key from file or create it if it doesn't exist yet.
        let rsa_private_pem = if rsa_private_path.is_file() {
            tracing::info!("found rsa key: {}", &rsa_private_path.display());

            let rsa_bytes = tokio::fs::read(rsa_private_path).await?;
            let rsa_pem = str::from_utf8(&rsa_bytes).map_err(|_| {
                tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, "invalid utf-8 sequence")
            })?;

            rsa_pem.to_string()
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

            rsa_pem.to_string()
        };

        Self::from_pem(&rsa_private_pem)
    }

    /// Create a new instance of the JWT factory from a rsa private pem.
    /// Pem is assumed to be pkcs8.
    pub fn from_pem(rsa_private_pem: &str) -> Result<Self, tokio::io::Error> {
        // Create private key so we can generate a public key.
        let private_key = rsa::RsaPrivateKey::from_pkcs8_pem(rsa_private_pem).map_err(|_| {
            tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, "invalid rsa pem")
        })?;

        let rsa_public_pem = private_key
            .to_public_key()
            .to_public_key_pem(rsa::pkcs8::LineEnding::default())
            .expect("failed to get public key pem");

        Ok(Self {
            key_private: jsonwebtoken::EncodingKey::from_rsa_pem(rsa_private_pem.as_bytes())
                .map_err(|e| tokio::io::Error::other(e.to_string()))?,
            key_public: jsonwebtoken::DecodingKey::from_rsa_pem(rsa_public_pem.as_bytes())
                .map_err(|e| tokio::io::Error::other(e.to_string()))?,
            issuer_type: PhantomData,
        })
    }

    /// Encode claims into signed JWT token.
    pub fn encode<T>(&self, claims: T) -> String
    where
        T: serde::ser::Serialize,
        T: JwtClaimMetadata,
    {
        let header = jsonwebtoken::Header::new(JWT_ALGORITHM);

        jsonwebtoken::encode(&header, &claims, &self.key_private)
            .expect("failed to encode jwt claims")
    }

    pub fn decode<T>(&self, token: &str) -> Result<T, jsonwebtoken::errors::Error>
    where
        T: serde::de::DeserializeOwned + Clone,
        T: JwtClaimMetadata,
    {
        let mut validator = jsonwebtoken::Validation::new(JWT_ALGORITHM);

        validator.validate_exp = true;
        validator.set_issuer(&[I::ISSUER]);
        validator.set_audience(&[T::AUDIENCE]);

        jsonwebtoken::decode(token, &self.key_public, &validator).map(|x| x.claims)
    }
}

#[cfg(test)]
mod tests {
    use crate::crypt::{JwtClaimAccess, JwtClaimMetadata, JwtFactory, JwtFactoryMetadata};

    const RSA_PEM_PKCS_1: &str = "-----BEGIN RSA PRIVATE KEY-----
MIIBOwIBAAJBAMh6bx7LiHQi5PTZ/jpyWIsMXJRZo68+4E3ngi6GAuBAKMyVKBdc
jW/22LA8rqRLDFxrV5wgQvMnvtJYrJ0QLqcCAwEAAQJAaRVECbBF5hokSPO6/ofR
QZFJNbmGwuUCTdN7uUclWsVZHnnzmHLnGUDjBDRi624bTnQDGQrc/s9Frbaq4jhb
AQIhAOK3UfoIyhmNh/EsLc+BQx7uiHLOLQZe1CglIVXCxUDfAiEA4l+GDlJ8/lin
7VILAMNG1X0IzJ8bBDpUv1vOG+JB4zkCIQDI9Sux0Jarfbtw9/MHSpGfWloSQVTB
n864YukgZouHywIgffDhFyTDT4opWvozDuiVhv66H4VBNZfyQEgmIhM9ztkCIQDg
hNbh3Sx/1KR0TZ0UpNac9NOXiJKq7XhXaHQlArMeSw==
-----END RSA PRIVATE KEY-----
";

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
    async fn from_pem() {
        let jwt_pkcs_1 = JwtFactory::<TestJwt>::from_pem(RSA_PEM_PKCS_1);
        let jwt_pkcs_8 = JwtFactory::<TestJwt>::from_pem(RSA_PEM_PKCS_8);

        assert!(jwt_pkcs_1.is_err());
        assert!(jwt_pkcs_8.is_ok());
    }

    #[tokio::test]
    async fn invalid() {
        let jwt = JwtFactory::<TestJwt>::from_pem(RSA_PEM_PKCS_8);

        assert!(jwt.is_ok());

        let jwt = jwt.unwrap();
        let uuid = uuid::Uuid::new_v4();

        let invalid_iss_jwt = jwt.encode(JwtClaimAccess::new("fart", uuid, "hello@mail.com"));
        let invalid_iss_jwt = jwt.decode::<JwtClaimAccess>(&invalid_iss_jwt);

        assert!(invalid_iss_jwt.is_err());

        let expired_jwt = jwt.encode(JwtClaimAccess {
            iss: TestJwt::ISSUER.into(),
            sub: "hello".into(),
            exp: 0,
            aud: JwtClaimAccess::AUDIENCE.into(),
            username: "hello@mail.com".into(),
        });

        let expired_jwt = jwt.decode::<JwtClaimAccess>(&expired_jwt);

        assert!(expired_jwt.is_err());
    }
}
