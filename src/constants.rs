use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

pub const BASE_ROUTE: &str = "/api/v1";
pub const IGNORED_AUTH_ROUTES: [&str; 3] = ["auth/register", "auth/login", "auth/refresh"];

//JWT Token part
pub const ISSUER: &str = "doc-storage-authenticator";
pub const EXPIRATION_TIME: u32 = 60 * 60 * 6; // 6 hours
pub const REFRESH_EXPIRATION_TIME: u32 = 60 * 60 * 24 * 90; // 90 days
lazy_static::lazy_static!(
    pub static ref HEADER: Header = Header::new(Algorithm::RS256);
    pub static ref VALIDATION: Validation = {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_required_spec_claims(&["exp", "iss"]);
        validation.set_issuer(&[ISSUER.to_string()]);
        validation
    };

    pub static ref ENCODING_KEY: EncodingKey = EncodingKey::from_rsa_pem(include_bytes!("../private.pem"))
        .expect("Failed to load private key. Is it present?");
    pub static ref REFRESH_ENCODING_KEY: EncodingKey = EncodingKey::from_rsa_pem(include_bytes!("../private.refresh.pem"))
            .expect("Failed to load refresh private key. Is it present?");
    pub static ref DECODING_KEY: DecodingKey = DecodingKey::from_rsa_pem(include_bytes!("../public.pem"))
        .expect("Failed to load public key. Is it present?");
    pub static ref REFRESH_DECODING_KEY: DecodingKey = DecodingKey::from_rsa_pem(include_bytes!("../public.refresh.pem"))
        .expect("Failed to load refresh public key. Is it present?");
);
