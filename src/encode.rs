use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
use aes::Aes256;
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use generic_array::GenericArray;

use crate::analysis::Analysis;

const KEY_SIZE: usize = 32;
const IV_SIZE: usize = 16;
const COMPRESSION_LEVEL: i32 = 10;

type Cipher = cbc::Encryptor<Aes256>;

pub fn encode(analysis: &Analysis, key_encoded: &str) -> Result<Vec<u8>> {
	// Marshal the analysis to JSON.
	let analysis_json = serde_json::to_vec(&analysis)?;

	// Compress with zstd.
	let compressed = zstd::stream::encode_all(analysis_json.as_slice(), COMPRESSION_LEVEL)?;

	// Precede the ciphertext with a random IV.
	let mut buf = vec![0u8; IV_SIZE + compressed.len() + 16 - compressed.len() % 16];
	let mut iv = [0u8; IV_SIZE];
	getrandom::getrandom(&mut iv)
		.map_err(|e| anyhow!("Unable to read secure random data for encryption: {:?}", e))?;
	buf[..IV_SIZE].clone_from_slice(&iv);
	let iv_arr = GenericArray::from_slice(&iv);

	// Decode key (golang []byte is marshalled with base64 std)
	let key = general_purpose::STANDARD.decode(key_encoded)?;
	let mut key_slice = [0u8; KEY_SIZE];
	key_slice.clone_from_slice(&key);
	let key_arr = GenericArray::from_slice(&key_slice);

	// Write the ciphertext.
	Cipher::new(key_arr, iv_arr)
		.encrypt_padded_b2b_mut::<Pkcs7>(&compressed, &mut buf[IV_SIZE..])
		.map_err(|e| anyhow!("Encryption error: {:?}", e))?;
	Ok(buf)
}
