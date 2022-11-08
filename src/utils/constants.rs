use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

pub const BASE_ROUTE: &str = "/api/v1";
pub const IGNORED_AUTH_ROUTES: [&str; 2] = ["auth/register", "auth/login"];

lazy_static::lazy_static!(
    pub static ref HEADER: Header = Header::new(Algorithm::RS256);
    pub static ref VALIDATION: Validation = {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[ISSUER.to_string()]);
        validation
    };
    pub static ref ENCODING_KEY: EncodingKey = EncodingKey::from_rsa_pem(include_bytes!("../../private.pem"))
        .expect("Failed to load private key. Is it present?");
    pub static ref DECODING_KEY: DecodingKey = DecodingKey::from_rsa_pem(include_bytes!("../../public.pem"))
        .expect("Failed to load public key. Is it present?");
);

pub const ISSUER: &str = "doc-storage-authenticator";
pub const EXPIRATION_TIME: usize = 60 * 60 * 24; // 24 hours
