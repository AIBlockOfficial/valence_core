use crate::crypto::sign_ed25519 as sign;
use crate::crypto::sign_ed25519::{PublicKey, Signature};
use serde::{Deserialize, Serialize};
use tracing::warn;

/// Function to validate the signature using Ed25519
///
/// ### Argeumnts
///
/// * `public_key` - The public key of the signer
/// * `msg` - The message that was signed
/// * `signature` - The signature of the message
pub fn validate_signature(public_key: &str, msg: &str, signature: &str) -> bool {
    let pk_decode = hex::decode(public_key);
    let sig_decode = hex::decode(signature);

    if pk_decode.is_err() || sig_decode.is_err() {
        return false;
    }

    let pk = PublicKey::from_slice(&pk_decode.unwrap_or_default());
    let signature = Signature::from_slice(&sig_decode.unwrap_or_default());

    if pk.is_none() || signature.is_none() {
        warn!("Failed to decode public key or signature");
        return false;
    }

    sign::verify_detached(
        &signature.unwrap_or_default(),
        msg.as_bytes(),
        &pk.unwrap_or_default(),
    )
}

/// Function to serialize data
pub fn serialize_data<T: Serialize>(data: &T) -> String {
    serde_json::to_string(data).unwrap_or_default()
}

/// Function to deserialize data
pub fn deserialize_data<T: for<'a> Deserialize<'a>>(data: String) -> T {
    match serde_json::from_str(&data) {
        Ok(result) => result,
        Err(_) => {
            warn!("Failed to deserialize data");
            serde_json::from_str("{}").unwrap()
        }
    }
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

    #[test]
    fn should_not_validate_incorrect_signature() {
        //
        // Arrange
        //
        let public_key = "2304667047f9ac074b2b062b46557e3283a1661d8348b3d6323490b9c8ccf978";
        let msg = "18f70e4a53a7cfd7f82d0e1fc287a449872ec7489dba0dff86144df8609caeda";
        let signature = "04e37549dcc0422237aa1e3c18b6a2cdb529831e8daad8ea4d3865cad149d50016e7b64cedf12c01fb5eb715b2e049bb3441eb6da9866ce1eef6071b38465808";

        //
        // Act
        //
        let result = validate_signature(public_key, msg, signature);

        //
        // Assert
        //
        assert!(!result);
    }
}
