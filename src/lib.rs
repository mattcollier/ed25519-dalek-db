use ed25519_dalek::{Keypair, PublicKey, Signer, Verifier, Signature};
use base58::{FromBase58, ToBase58};
use node_bindgen::derive::node_bindgen;
use node_bindgen::core::buffer::ArrayBuffer;
use node_bindgen::core::NjError;

#[node_bindgen]
pub fn sign(private_key_bytes: &[u8], message: &[u8]) -> Result<String,NjError> {
  let keypair: Keypair = Keypair::from_bytes(private_key_bytes).unwrap();
  ArrayBuffer::new(keypair.sign(message).to_bytes().to_vec());
  Ok("foo".to_string())
}

// pub fn sign_x(private_key_bytes: &[u8], message: &[u8], ) -> [u8; 64] {
//   let keypair: Keypair = Keypair::from_bytes(private_key_bytes).unwrap();
//   keypair.sign(message).to_bytes()
// }
// pub fn verify(public_key_bytes: &[u8], message: &[u8], ) -> bool {
//   let s: [u8; 64] = signature_bytes.try_into().unwrap();

//   // let t = format!("{:?}", s);
//   // console::log_2(&"SIGNATURE".into(), &t.into());
//   let signature = ed25519_dalek::Signature::new(s);
//   // let u = format!("{:?}", signature.to_bytes());
//   // console::log_2(&"SIGNATURE_to_bytes".into(), &u.into());
//   let verify_key = PublicKey::from_bytes(public_key_bytes).unwrap();

// }

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
