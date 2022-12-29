use argon2::Config;

pub fn hash_string(input: &str) -> String {
    let salt = b"secret_salt";
    argon2::hash_encoded(input.as_ref(), salt, &Config::default()).unwrap()
}

pub fn hash_verify(input: &str, encoding: &str) -> bool {
    let hashed_input = hash_string(input);
    return hashed_input.as_str() == encoding;
}

#[cfg(test)]
mod helper_test {
    use super::*;

    #[test]
    fn checks_verify_hash_works_as_expected() -> () {
        let input = "Password1";
        let hashed_input = hash_string(input);

        let is_matching = hash_verify(input, hashed_input.as_str());
        assert_eq!(is_matching, true);
    }
}
