use crate::crypto::sign_ed25519 as sign;
use crate::crypto::sign_ed25519::{PublicKey, Signature};
use serde::{Deserialize, Serialize};

/// Function to validate the signature using Ed25519
///
/// ### Argeumnts
///
/// * `public_key` - The public key of the signer
/// * `msg` - The message that was signed
/// * `signature` - The signature of the message
pub fn validate_signature(public_key: &str, msg: &str, signature: &str) -> bool {
    let pk_decode = hex::decode(public_key).expect("Decoding failed");
    let sig_decode = hex::decode(signature).expect("Decoding failed");

    let pk = PublicKey::from_slice(&pk_decode).unwrap();
    let signature = Signature::from_slice(&sig_decode).unwrap();

    sign::verify_detached(&signature, msg.as_bytes(), &pk)
}

/// Function to serialize data
pub fn serialize_data<T: Serialize>(data: &T) -> String {
    serde_json::to_string(data).unwrap()
}

/// Function to deserialize data
pub fn deserialize_data<T: for<'a> Deserialize<'a>>(data: String) -> T {
    serde_json::from_str(&data).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_validate_correct_signature() {
        //
        // Arrange
        //
        let public_key = "a1c03c87549f1cb6f277f24e65111736c1faa8c47d20f3cf8ebaa595c252503c";
        let msg = "Hello World!";
        let signature = "f4873d6b68c1b09779bf0290d70322f1e7c8e5cf9a1f5c7cec97bb47623e122b944030bebfff82a3c991230a5c2c26ee0b71c2824b80069e5c5c0b702e69bf01";

        //
        // Act
        //
        let result = validate_signature(public_key, msg, signature);

        //
        // Assert
        //
        assert!(result);
    }
}
