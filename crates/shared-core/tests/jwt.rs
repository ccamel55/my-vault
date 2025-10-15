use shared_core::crypt::{JwtClaimAccess, JwtClaimRefresh, JwtFactory};

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

const ISSUER: &str = "test";

#[tokio::test]
async fn from_pem() {
    let jwt_pkcs_1 = JwtFactory::from_pem(ISSUER, RSA_PEM_PKCS_1);
    let jwt_pkcs_8 = JwtFactory::from_pem(ISSUER, RSA_PEM_PKCS_8);

    assert!(jwt_pkcs_1.is_err());
    assert!(jwt_pkcs_8.is_ok());
}

#[tokio::test]
async fn encode_claim() {
    let jwt = JwtFactory::from_pem(ISSUER, RSA_PEM_PKCS_8);

    assert!(jwt.is_ok());

    let jwt = jwt.unwrap();

    let access_jwt = jwt.encode(JwtClaimAccess {
        iss: ISSUER.into(),
        sub: "hello".into(),
        exp: 123,
        email: "hello@mail.me".into(),
    });

    assert_eq!(
        access_jwt,
        "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.\
         eyJpc3MiOiJ0ZXN0Iiwic3ViIjoiaGVsbG8iL\
         CJleHAiOjEyMywiZW1haWwiOiJoZWxsb0BtYW\
         lsLm1lIn0.byaA7LJsfYplZyXFbPRPo8AWmqp\
         KC09QkK3VWXY4tvPo7yNsK-_lXMYKT3hjNwo9\
         dnbuUkLkKLBxYbn4vZ58Rw"
    );

    let refresh_jwt = jwt.encode(JwtClaimRefresh {
        iss: ISSUER.into(),
        sub: "hello".into(),
        exp: 456,
    });

    assert_eq!(
        refresh_jwt,
        "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.\
         eyJpc3MiOiJ0ZXN0Iiwic3ViIjoiaGVsbG8iL\
         CJleHAiOjQ1Nn0.PMXHODPFlxzAZkmW0Kv09X\
         cKFcUqIyqohbPyI_r6M_u73wqapUymn5VL2YE\
         Eh20Rhs2n1X6LOXBggaPmtBfF3w"
    );
}

#[tokio::test]
async fn encode_decode_claim() {
    let jwt = JwtFactory::from_pem(ISSUER, RSA_PEM_PKCS_8);

    assert!(jwt.is_ok());

    let jwt = jwt.unwrap();

    let access_jwt = jwt.encode(JwtClaimAccess::new(ISSUER, "hello", "hello@mail.com"));
    let access_jwt = jwt.decode::<JwtClaimAccess>(&access_jwt);

    assert!(access_jwt.is_ok());

    let access_jwt = access_jwt.unwrap();

    assert_eq!(access_jwt.iss, ISSUER);
    assert_eq!(access_jwt.sub, "hello");
    assert_eq!(access_jwt.email, "hello@mail.com");

    let refresh_jwt = jwt.encode(JwtClaimRefresh::new(ISSUER, "hello"));
    let refresh_jwt = jwt.decode::<JwtClaimRefresh>(&refresh_jwt);

    assert!(refresh_jwt.is_ok());

    let refresh_jwt = refresh_jwt.unwrap();

    assert_eq!(refresh_jwt.iss, ISSUER);
    assert_eq!(refresh_jwt.sub, "hello");
}

#[tokio::test]
async fn invalid() {
    let jwt = JwtFactory::from_pem(ISSUER, RSA_PEM_PKCS_8);

    assert!(jwt.is_ok());

    let jwt = jwt.unwrap();

    let invalid_iss_jwt = jwt.encode(JwtClaimAccess::new("fart", "hello", "hello@mail.com"));
    let invalid_iss_jwt = jwt.decode::<JwtClaimAccess>(&invalid_iss_jwt);

    assert!(invalid_iss_jwt.is_err());

    let expired_jwt = jwt.encode(JwtClaimAccess {
        iss: ISSUER.into(),
        sub: "hello".into(),
        exp: 0,
        email: "hello@mail.com".into(),
    });

    let expired_jwt = jwt.decode::<JwtClaimAccess>(&expired_jwt);

    assert!(expired_jwt.is_err());
}
