use sha2::{Digest, Sha256};

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod test {
    use crate::util::hash_password;

    #[test]
    fn test_encrypt_password() {
        const PASSWORD: &str = "password";

        assert_eq!(
            hash_password(PASSWORD),
            "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8"
        );
    }
}
