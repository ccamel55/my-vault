use argon2::PasswordHasher;
use std::sync::Arc;

/// Default argon2 algorithm.
const DEFAULT_ARGON2_ALGORITHM: argon2::Algorithm = argon2::Algorithm::Argon2id;

/// Argon2 factory
#[derive(Debug)]
pub struct Argon2Factory {
    argon: Arc<argon2::Argon2<'static>>,
}

impl Argon2Factory {
    /// Create new argon 2 factory.
    pub fn new(iters: u32, memory_mb: u32, parallelism: u32) -> Result<Self, argon2::Error> {
        let memory_kb = memory_mb * (2 ^ 10);

        let params = argon2::Params::new(memory_kb, iters, parallelism, None)?;
        let argon =
            argon2::Argon2::new(DEFAULT_ARGON2_ALGORITHM, argon2::Version::default(), params);

        Ok(Self {
            argon: Arc::new(argon),
        })
    }

    /// Encode passphrase
    pub async fn encode(
        &self,
        passphrase: &[u8],
        salt: &[u8],
    ) -> Result<String, crate::error::Error> {
        let argon = self.argon.clone();

        tokio::task::spawn_blocking({
            let passphrase = passphrase.to_vec();
            let salt = argon2::password_hash::SaltString::encode_b64(salt)
                .map_err(|_| crate::error::Error::Crypto)?;

            move || {
                argon
                    .hash_password(&passphrase, &salt)
                    .map(|x| x.to_string())
                    .map_err(|_| crate::error::Error::Crypto)
            }
        })
        .await
        .map_err(crate::error::Error::TaskJoin)?
    }
}

impl Default for Argon2Factory {
    fn default() -> Self {
        Self::new(2, 32, 2).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn invalid_args() {
        let result_ok = super::Argon2Factory::new(2, 32, 2);

        assert!(result_ok.is_ok());

        let result_invalid_iters = super::Argon2Factory::new(0, 32, 2);

        assert!(result_invalid_iters.is_err());

        let result_invalid_memory = super::Argon2Factory::new(2, 0, 2);

        assert!(result_invalid_memory.is_err());

        let result_invalid_parallelism = super::Argon2Factory::new(2, 32, 0);

        assert!(result_invalid_parallelism.is_err());
    }

    #[tokio::test]
    async fn encode() {
        let argon = super::Argon2Factory::new(2, 32, 2);

        assert!(argon.is_ok());

        let argon = argon.unwrap();

        const PASSPHRASE: &str = "hello world!";

        let salt_1 = b"my salt sucks";
        let salt_2 = b"you salt sucks too";

        let result_1 = argon.encode(PASSPHRASE.as_bytes(), salt_1).await;
        let result_2 = argon.encode(PASSPHRASE.as_bytes(), salt_2).await;

        assert!(result_1.is_ok());
        assert!(result_2.is_ok());
    }
}
