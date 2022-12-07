// Specify linters to Vigeners module.
// TODO(Cardosaum): Enable all these linters
// #![cfg_attr(
// 	not(test),
// 	deny(
// 		clippy::all,
// 		clippy::cargo,
// 		clippy::complexity,
// 		clippy::correctness,
// 		clippy::nursery,
// 		// clippy::pedantic,
// 		clippy::perf,
// 		// clippy::restriction,
// 		clippy::style,
// 		clippy::suspicious,
// 		// missing_docs,
// 		rustdoc::missing_crate_level_docs,
// 		rustdoc::missing_doc_code_examples,
// 		warnings,
// 	)
// )]

use std::{ascii::AsciiExt, ops::Sub};

// TODO(Cardosaum): Remove missing docs
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("Invalid key! (Key must contain alphabetic characters only)")]
    InvalidKey,
}

// TODO(Cardosaum): Remove missing docs
#[allow(missing_docs)]
pub type Result<T> = std::result::Result<T, Error>;
// TODO(Cardosaum): Search how to instantiate a `Key` only if it passes the validity check. Something similar is done in Gamercode (?)
pub type Filter = Vec<u8>;

pub trait Vigeners {
    // TODO(Cardosaum): Write documentation for all methods

    // === Cipher ===
    fn encipher(&self, key: &str) -> Result<String>;
    // fn decipher(&self, key: &str) -> Result<String>;

    // === Key ===
    fn parse_key(&self) -> Option<Filter>;
    fn validate_key(&self) -> Result<Filter>;

    // === Helpers ===
    // fn transpose(&self, filter: &[u8]) -> Result<String>;
}

impl Vigeners for str {
    fn encipher(&self, key: &str) -> Result<String> {
        // Sanity checks
        let key = key.validate_key()?;

        Ok(self.to_string())
    }

    fn parse_key(&self) -> Option<Filter> {
        let mut key = Vec::with_capacity(self.chars().size_hint().1.unwrap_or_default());
        for c in self.chars() {
            match (
                c.is_alphabetic(),
                u8::try_from(c.to_ascii_lowercase())
                    .and_then(|x| Ok(x.max(65).sub(x.min(65))))
                    .ok(),
            ) {
                (true, Some(v)) => key.push(v),
                _ => return None,
            }
        }
        return Some(key);
    }

    fn validate_key(&self) -> Result<Filter> {
        match self.parse_key() {
            Some(key) => Ok(key),
            _ => Err(Error::InvalidKey),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_key() {
        let key = "mykey";
        assert!(key.validate_key())
    }

    #[test]
    fn is_invalid_key() {
        let key = "my key";
        assert!(!key.validate_key())
    }

    #[test]
    fn succeed_enchiper_text() {
        let key = "mykey";
        let text = "I'm just testing this cipher :)";
        assert_eq!(Ok(text.to_string()), text.encipher(key))
    }
}
