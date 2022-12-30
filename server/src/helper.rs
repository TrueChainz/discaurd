use argon2::Config;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::actors::user_actor::UserClaims;

pub enum TokenType {
    AccessToken,
    RefreshToken,
}

pub fn hash_string(input: &str) -> String {
    let salt = b"secret_salt";
    argon2::hash_encoded(input.as_ref(), salt, &Config::default()).unwrap()
}

pub fn hash_verify(input: &str, encoding: &str) -> bool {
    let hashed_input = hash_string(input);
    return hashed_input.as_str() == encoding;
}

pub fn generate_token(token_claims: &UserClaims, token_type: TokenType) -> String {
    let access_encode_key = EncodingKey::from_secret("access_token_key".as_ref());
    let refresh_encode_key = EncodingKey::from_secret("refresh_token_key".as_ref());

    return match token_type {
        TokenType::AccessToken => {
            encode(&Header::default(), &token_claims, &access_encode_key).unwrap()
        }
        TokenType::RefreshToken => {
            encode(&Header::default(), &token_claims, &refresh_encode_key).unwrap()
        }
    };
}

pub fn validate_token(token: String, token_type: TokenType) -> bool {
    let access_encode_key = DecodingKey::from_secret("access_token_key".as_ref());
    let refresh_encode_key = DecodingKey::from_secret("refresh_token_key".as_ref());

    let validate_results = match token_type {
        TokenType::AccessToken => {
            decode::<UserClaims>(&token, &access_encode_key, &Validation::default())
        }
        TokenType::RefreshToken => {
            decode::<UserClaims>(&token, &refresh_encode_key, &Validation::default())
        }
    };

    return validate_results.is_ok();
}

#[cfg(test)]
mod helper_test {
    use super::*;
    use chrono::{Duration, Utc};
    use std::ops::Add;

    #[test]
    fn checks_verify_hash_works() {
        let input = "Password1";
        let hashed_input = hash_string(input);

        let is_matching = hash_verify(input, hashed_input.as_str());
        assert_eq!(is_matching, true);
    }

    #[test]
    fn checks_generate_token_works() {
        let token_claim = UserClaims {
            id: "asdasd".to_string(),
            username: "asdasd".to_string(),
            exp: 12,
        };
        let encoded_access_token = generate_token(&token_claim, TokenType::AccessToken);

        assert_eq!(encoded_access_token, encoded_access_token);

        let encoded_refresh_token = generate_token(&token_claim, TokenType::RefreshToken);

        assert_eq!(encoded_refresh_token, encoded_refresh_token);
    }

    #[test]
    fn checks_validate_token_works() {
        // ARRANGE
        let token_claim = UserClaims {
            id: "asdasd".to_string(),
            username: "asdasd".to_string(),
            exp: Utc::now().add(Duration::minutes(30)).timestamp() as usize, // exp: 123123213,
        };
        let expired_token_claim = UserClaims {
            id: "asdasd".to_string(),
            username: "asdasd".to_string(),
            exp: Utc::now().add(Duration::minutes(-30)).timestamp() as usize, // exp: 123123213,
        };

        // ACT
        let access_token = generate_token(&token_claim, TokenType::AccessToken);
        let expired_access_token = generate_token(&expired_token_claim, TokenType::AccessToken);
        let refresh_token = generate_token(&token_claim, TokenType::RefreshToken);
        let expired_refresh_token = generate_token(&expired_token_claim, TokenType::RefreshToken);

        // ASSERT
        assert_eq!(
            validate_token(access_token, TokenType::AccessToken),
            true,
            "With a valid access token, validate token should return true"
        );
        assert_eq!(
            validate_token(refresh_token, TokenType::RefreshToken),
            true,
            "With a valid refresh token, validate token should return true"
        );
        assert_eq!(
            validate_token(expired_access_token, TokenType::AccessToken),
            false,
            "With an expired access token, validate token should return true"
        );
        assert_eq!(
            validate_token(expired_refresh_token, TokenType::RefreshToken),
            false,
            "With an expired access token, validate token should return true"
        );
    }
}
