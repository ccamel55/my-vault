pub mod uuid {
    /// Implement deserialize for uuid::fmt::Hyphenated
    #[derive(Debug, serde::Deserialize)]
    #[serde(remote = "uuid::fmt::Hyphenated")]
    pub struct Hyphenated(#[serde(getter = "uuid::fmt::Hyphenated::into_uuid")] uuid::Uuid);

    impl From<Hyphenated> for uuid::fmt::Hyphenated {
        fn from(def: Hyphenated) -> uuid::fmt::Hyphenated {
            def.0.hyphenated()
        }
    }

    /// Implement deserialize for uuid::fmt::Simple
    #[derive(Debug, serde::Deserialize)]
    #[serde(remote = "uuid::fmt::Simple")]
    pub struct Simple(#[serde(getter = "uuid::fmt::Simple::into_uuid")] uuid::Uuid);

    impl From<Simple> for uuid::fmt::Simple {
        fn from(def: Simple) -> uuid::fmt::Simple {
            def.0.simple()
        }
    }

    /// Implement deserialize for uuid::fmt::Urn
    #[derive(Debug, serde::Deserialize)]
    #[serde(remote = "uuid::fmt::Urn")]
    pub struct Urn(#[serde(getter = "uuid::fmt::Urn::into_uuid")] uuid::Uuid);

    impl From<Urn> for uuid::fmt::Urn {
        fn from(def: Urn) -> uuid::fmt::Urn {
            def.0.urn()
        }
    }

    /// Implement deserialize for uuid::fmt::Braced
    #[derive(Debug, serde::Deserialize)]
    #[serde(remote = "uuid::fmt::Braced")]
    pub struct Braced(#[serde(getter = "uuid::fmt::Braced::into_uuid")] uuid::Uuid);

    impl From<Braced> for uuid::fmt::Braced {
        fn from(def: Braced) -> uuid::fmt::Braced {
            def.0.braced()
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    /// Test UUID
    const UUID_TEST: u128 = 208379411211518731371528733611289964568;

    #[tokio::test]
    async fn hyphenated() {
        #[derive(Serialize, Deserialize)]
        struct Test {
            #[serde(deserialize_with = "super::uuid::Hyphenated::deserialize")]
            a: uuid::fmt::Hyphenated,
        }

        let test_1 = Test {
            a: uuid::Uuid::from_u128(UUID_TEST).hyphenated(),
        };

        let json = serde_json::to_string(&test_1);

        assert!(json.is_ok());

        let json = json.unwrap();

        assert_eq!(json, "{\"a\":\"9cc46a2f-52e7-4e93-899b-c1b3108b7018\"}");
    }

    #[tokio::test]
    async fn simple() {
        #[derive(Serialize, Deserialize)]
        struct Test {
            #[serde(deserialize_with = "super::uuid::Simple::deserialize")]
            a: uuid::fmt::Simple,
        }

        let test_1 = Test {
            a: uuid::Uuid::from_u128(UUID_TEST).simple(),
        };

        let json = serde_json::to_string(&test_1);

        assert!(json.is_ok());

        let json = json.unwrap();

        assert_eq!(json, "{\"a\":\"9cc46a2f52e74e93899bc1b3108b7018\"}");
    }

    #[tokio::test]
    async fn urn() {
        #[derive(Serialize, Deserialize)]
        struct Test {
            #[serde(deserialize_with = "super::uuid::Urn::deserialize")]
            a: uuid::fmt::Urn,
        }

        let test_1 = Test {
            a: uuid::Uuid::from_u128(UUID_TEST).urn(),
        };

        let json = serde_json::to_string(&test_1);

        assert!(json.is_ok());

        let json = json.unwrap();

        assert_eq!(
            json,
            "{\"a\":\"urn:uuid:9cc46a2f-52e7-4e93-899b-c1b3108b7018\"}"
        );
    }

    #[tokio::test]
    async fn braced() {
        #[derive(Serialize, Deserialize)]
        struct Test {
            #[serde(deserialize_with = "super::uuid::Braced::deserialize")]
            a: uuid::fmt::Braced,
        }

        let test_1 = Test {
            a: uuid::Uuid::from_u128(UUID_TEST).braced(),
        };

        let json = serde_json::to_string(&test_1);

        assert!(json.is_ok());

        let json = json.unwrap();

        assert_eq!(json, "{\"a\":\"{9cc46a2f-52e7-4e93-899b-c1b3108b7018}\"}");
    }
}
