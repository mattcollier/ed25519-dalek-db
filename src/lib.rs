use ed25519_dalek::{Keypair, PublicKey, Signer, Verifier, Signature};
use node_bindgen::derive::node_bindgen;
use node_bindgen::core::buffer::ArrayBuffer;
use std::convert::TryInto;

#[node_bindgen]
pub fn sign(private_key_bytes:  &[u8], message_bytes:  &[u8]) ->  ArrayBuffer {
    let keypair: Keypair = Keypair::from_bytes(private_key_bytes).unwrap();
    ArrayBuffer::new(keypair.sign(message_bytes).to_bytes().to_vec())
}

#[node_bindgen]
pub fn verify(message: &[u8], public_key_bytes: &[u8], signature_bytes: &[u8], ) -> bool {
    let signature = Signature::new(signature_bytes.try_into().unwrap());
    let verify_key = PublicKey::from_bytes(public_key_bytes).unwrap();
    verify_key.verify(message, &signature).is_ok()
}

// #[test]
// fn foo() {
//   let target_signature_base58 = "4AbhYFuwyJd3zPbqR6HieQPdz2DWK2k926v99AegFT9bMRKoagq5be7edGQDhguu37qVw3ULE5fh4ZCTZEYNKxaM".to_string();
//   let private_key_base58 = "sSicNq6YBSzafzYDAcuduRmdHtnrZRJ7CbvjzdQhC45ewwvQeuqbM2dNwS9RCf6buUJGu6N3rBy6oLSpMwha8tc".to_string();
//   let key_bytes = private_key_base58.from_base58().unwrap();
//   // convert vec u8 to &[u8]
//   let k: &[u8] = &key_bytes;
//   let s = sign(k, "test 1234".as_bytes());
//   let signature_base58 = s.to_base58();
//   assert_eq!(signature_base58, target_signature_base58);
// }
