use std::fs;
use std::path::Path;

pub trait Cipher {
    fn encrypt(&self, plaintext: &str) -> String;
    fn decrypt(&self, ciphertext: &str) -> String;
    fn repeat_string(s: &str, desired_length: usize) -> String;
}

#[derive(Debug)]
pub struct VigenereCipher {
    pub key: String,
}

impl Cipher for VigenereCipher {
    fn encrypt(&self, plaintext: &str) -> String {
        let plaintext_int: Vec<u8> = plaintext.to_lowercase().bytes().map(|b| b - 97).collect();
        let key = Self::repeat_string(&self.key.to_lowercase(), plaintext_int.len());
        let key_length = key.len();
        let key_as_int: Vec<u8> = key.bytes().map(|b| b - 97).collect();

        let mut ciphertext = String::new();
        for i in 0..plaintext_int.len() {
            let value = (plaintext_int[i] + key_as_int[i % key_length]) % 26;
            ciphertext.push((value + 97) as char);
        }
        ciphertext
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        let ciphertext_int: Vec<u8> = ciphertext.to_lowercase().bytes().map(|b| b - 97).collect();
        let key = Self::repeat_string(&self.key.to_lowercase(), ciphertext_int.len());
        let key_length = key.len();
        let key_as_int: Vec<u8> = key.bytes().map(|b| b - 97).collect();

        let mut plaintext = String::new();
        for i in 0..ciphertext_int.len() {
            dbg!(&i, &ciphertext_int, &key_as_int, &ciphertext_int[i], &key_length, i % key_length, &key_as_int[i & key_length]);
            let value = (ciphertext_int[i] - key_as_int[i % key_length]) % 26;
            plaintext.push((value + 97) as char);
        }
        plaintext
    }

    fn repeat_string(s: &str, desired_length: usize) -> String {
        let mut result = s.to_string();
        while result.len() < desired_length {
            result.push_str(s);
        }
        result.truncate(desired_length);
        result
    }
}

pub struct TestFileInput {
    path: String,
}

impl TestFileInput {
    pub fn new(path: &str) -> TestFileInput {
        TestFileInput {
            path: path.to_string(),
        }
    }

    pub fn read_contents(&self) -> String {
        let path = Path::new(&self.path);
        fs::read_to_string(path).expect("Error reading file")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_vigenere_cipher(key in "[a-z]{1,20}", plaintext in "[a-z]{1,20}") {
            dbg!("=================================================");
            dbg!(&key, &plaintext );
            let cipher = VigenereCipher { key };
            let ciphertext = cipher.encrypt(&plaintext);
            let decrypted_message = cipher.decrypt(&ciphertext);
            dbg!(&cipher, &ciphertext, &decrypted_message);
            assert_eq!(plaintext, decrypted_message);
            dbg!("-------------------------------------------------");
        }
    }
}
