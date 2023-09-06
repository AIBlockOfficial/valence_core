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
