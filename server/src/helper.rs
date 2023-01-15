use argon2::Config;

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
