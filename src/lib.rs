//! In Module 1, we discussed Block ciphers like AES. Block ciphers have a fixed length input.
//! Real wold data that we wish to encrypt _may_ be exactly the right length, but is probably not.
//! When your data is too short, you can simply pad it up to the correct length.
//! When your data is too long, you have some options.
//!
//! In this exercise, we will explore a few of the common ways that large pieces of data can be
//! broken up and combined in order to encrypt it with a fixed-length block cipher.
//!
//! WARNING: ECB MODE IS NOT SECURE.
//! Seriously, ECB is NOT secure. Don't use it irl. We are implementing it here to understand _why_
//! it is not secure and make the point that the most straight-forward approach isn't always the
//! best, and can sometimes be trivially broken.

use aes::{
	cipher::{generic_array::GenericArray, BlockCipher, BlockDecrypt, BlockEncrypt, KeyInit},
	Aes128,
};
use rand::{distributions::Alphanumeric, Rng};

///We're using AES 128 which has 16-byte (128 bit) blocks.
const BLOCK_SIZE: usize = 16;

/// Simple AES encryption
/// Helper function to make the core AES block cipher easier to understand.
fn aes_encrypt(data: [u8; BLOCK_SIZE], key: &[u8; BLOCK_SIZE]) -> [u8; BLOCK_SIZE] {
	// Convert the inputs to the necessary data type
	let mut block = GenericArray::from(data);
	let key = GenericArray::from(*key);

	let cipher = Aes128::new(&key);

	cipher.encrypt_block(&mut block);

	block.into()
}

fn string_to_u8_16(s: &str) -> [u8; BLOCK_SIZE] {
    let mut array = [0u8; BLOCK_SIZE];
    let bytes = s.as_bytes();
    let len = bytes.len().min(BLOCK_SIZE);
    array[..len].copy_from_slice(&bytes[..len]);
    array
}

fn vec_u8_to_u8_16(data: Vec<u8>)-> [u8; BLOCK_SIZE] {
	let mut array = [0u8; BLOCK_SIZE];
    let len = data.len().min(BLOCK_SIZE);
   	array[..len].copy_from_slice(&data[..len]);

	array
}

/// Simple AES encryption
/// Helper function to make the core AES block cipher easier to understand.
fn aes_decrypt(data: [u8; BLOCK_SIZE], key: &[u8; BLOCK_SIZE]) -> [u8; BLOCK_SIZE] {
	// Convert the inputs to the necessary data type
	let mut block = GenericArray::from(data);
	let key = GenericArray::from(*key);

	let cipher = Aes128::new(&key);

	cipher.decrypt_block(&mut block);

	block.into()
}

/// Before we can begin encrypting our raw data, we need it to be a multiple of the
/// block length which is 16 bytes (128 bits) in AES128.
///
/// The padding algorithm here is actually not trivial. The trouble is that if we just
/// naively throw a bunch of zeros on the end, there is no way to know, later, whether
/// those zeros are padding, or part of the message, or some of each.
///
/// The scheme works like this. If the data is not a multiple of the block length,  we
/// compute how many pad bytes we need, and then write that number into the last several bytes.
/// Later we look at the last byte, and remove that number of bytes.
///
/// But if the data _is_ a multiple of the block length, then we have a problem. We don't want
/// to later look at the last byte and remove part of the data. Instead, in this case, we add
/// another entire block containing the block length in each byte. In our case,
/// [16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16]
fn pad(mut data: Vec<u8>) -> Vec<u8> {
	// When twe have a multiple the second term is 0
	let number_pad_bytes = BLOCK_SIZE - data.len() % BLOCK_SIZE;

	for _ in 0..number_pad_bytes {
		data.push(number_pad_bytes as u8);
	}

	data
}

/// Groups the data into BLOCK_SIZE blocks. Assumes the data is already
/// a multiple of the block size. If this is not the case, call `pad` first.
fn group(data: Vec<u8>) -> Vec<[u8; BLOCK_SIZE]> {
	let mut blocks = Vec::new();
	let mut i = 0;
	while i < data.len() {
		let mut block: [u8; BLOCK_SIZE] = Default::default();
		block.copy_from_slice(&data[i..i + BLOCK_SIZE]);
		blocks.push(block);

		i += BLOCK_SIZE;
	}

	blocks
}

/// Does the opposite of the group function
fn un_group(blocks: Vec<[u8; BLOCK_SIZE]>) -> Vec<u8> {
    let mut _block : Vec<u8> = Vec::new();
	for block in blocks.iter() {
		for val in block.iter() {
			_block.push(val.clone())
		}
    }
	_block
}

/// Does the opposite of the pad function.
fn un_pad(data: Vec<u8>) -> Vec<u8> {
    let mut block: Vec<u8> = Vec::new();
	let mut i = 0;
	for _ in 0..BLOCK_SIZE {

		if data[i] >= 0 && data[i] <= 16 {
			block.push(data[i])
		}
		i += 1;
	}

	data
}
/// The first mode we will implement is the Electronic Code Book, or ECB mode.
/// Warning: THIS MODE IS NOT SECURE!!!!
///
/// This is probably the first thing you think of when considering how to encrypt
/// large data. In this mode we simply encrypt each block of data under the same key.
/// One good thing about this mode is that it is parallelizable. But to see why it is
/// insecure look at: https://www.ubiqsecurity.com/wp-content/uploads/2022/02/ECB2.png
fn ecb_encrypt(plain_text: Vec<u8>, key: [u8; 16]) -> Vec<u8> {
	let data = vec_u8_to_u8_16(plain_text);
	let e = aes_encrypt(data, &key);
	e.to_vec()
}

/// Opposite of ecb_encrypt.
fn ecb_decrypt(cipher_text: Vec<u8>, key: [u8; BLOCK_SIZE]) -> Vec<u8> {
	let data = vec_u8_to_u8_16(cipher_text);
	let e = aes_decrypt(data, &key);
	e.to_vec()
}

/// The next mode, which you can implement on your own is cipherblock chaining.
/// This mode actually is secure, and it often used in real world applications.
///
/// In this mode, the ciphertext from the first block is XORed with the
/// plaintext of the next block before it is encrypted.
///
/// For more information, and a very clear diagram,
/// see https://de.wikipedia.org/wiki/Cipher_Block_Chaining_Mode
///
/// You will need to generate a random initialization vector (IV) to encrypt the
/// very first block because it doesn't have a previous block. Typically this IV
/// is inserted as the first block of ciphertext.
fn cbc_encrypt(plain_text: Vec<u8>, key: [u8; BLOCK_SIZE]) -> Vec<u8> {
	// Remember to generate a random initialization vector for the first block.
	let padded_data = pad(plain_text.to_vec());
	let data = vec_u8_to_u8_16(padded_data);
	let aes_encrypted_data = aes_encrypt(data, &key);
	aes_encrypted_data.to_vec()
}

fn cbc_decrypt(cipher_text: Vec<u8>, key: [u8; BLOCK_SIZE]) -> Vec<u8> {

	let data = vec_u8_to_u8_16(cipher_text);
	let aes_decrypt_data = aes_decrypt(data, &key);
	let un_pad_data = un_pad(aes_decrypt_data.to_vec());
	un_pad_data
}

/// Another mode which you can implement on your own is counter mode.
/// This mode is secure as well, and is used in real world applications.
/// It allows parallelized encryption and decryption, as well as random read access when decrypting.
///
/// In this mode, there is an index for each block being encrypted (the "counter"), as well as a random nonce.
/// For a 128-bit cipher, the nonce is 64 bits long.
///
/// For the ith block, the 128-bit value V of `nonce | counter` is constructed, where | denotes
/// concatenation. Then, V is encrypted with the key using ECB mode. Finally, the encrypted V is
/// XOR'd with the plaintext to produce the ciphertext.
///
/// A very clear diagram is present here:
/// https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation#Counter_(CTR)
///
/// Once again, you will need to generate a random nonce which is 64 bits long. This should be
/// inserted as the first block of the ciphertext.
fn ctr_encrypt(plain_text: Vec<u8>, key: [u8; BLOCK_SIZE]) -> Vec<u8> {
    let mut random_nounce: Vec<u8> = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(64)
        .collect();
    let mut counter = 0usize;
    let mut cipher = Vec::new();
    plain_text.chunks(128).for_each(|chunk| {
        let counter_array = counter.to_le_bytes()[..].to_vec();
        counter += 1;

        random_nounce.extend(counter_array);
        let stub = pad(random_nounce.clone());

        let sub_cipher = ecb_encrypt(stub, key);
        let xor = sub_cipher
            .iter()
            .zip(chunk)
            .map(|(x, y)| x ^ y)
            .collect::<Vec<u8>>();
        cipher.extend(xor);
    });
    cipher
}

fn ctr_decrypt(cipher_text: Vec<u8>, key: [u8; BLOCK_SIZE]) -> Vec<u8> {
    let mut random_nounce: Vec<u8> = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(64)
        .collect();
    let mut counter = 0usize;
    let mut cipher = Vec::new();
    cipher_text.chunks(128).for_each(|chunk| {
        let counter_array = counter.to_le_bytes()[..].to_vec();
        counter += 1;

        random_nounce.extend(counter_array);
        let stub = pad(random_nounce.clone());

        let sub_cipher = ecb_encrypt(stub, key);
        let xor = sub_cipher
            .iter()
            .zip(chunk)
            .map(|(x, y)| x ^ y)
            .collect::<Vec<u8>>();
        cipher.extend(xor);
    });
    cipher
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn un_group_test() {
		let plaintext = "Hello, world!";
		
		let data = string_to_u8_16(plaintext);

		let padded = pad(data.to_vec());

		let group_data = group(padded);

        assert_eq!(
            vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 0, 0, 0, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16],
            un_group(group_data)
        )
    }

	#[test]
    fn un_pad_test() {
		let plaintext = "Hello, world!";
		
		let data = string_to_u8_16(plaintext);

		let padded = pad(data.to_vec());

        assert_eq!(
            vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 0, 0, 0],
            un_pad(padded)
        )
    }

	#[test]
    fn ecb_encrypt_test() {
		let plaintext = "Hello, world!";
		let key = "PBA";
		let _key = string_to_u8_16(key);
		let data = string_to_u8_16(plaintext);

        assert_eq!(
            vec![211, 31, 103, 243, 12, 56, 41, 155, 23, 60, 70, 227, 13, 165, 132, 46],
            ecb_encrypt(data.to_vec(), _key)
        )
    }

	#[test]
    fn ecb_decrypt_test() {
		let plaintext = "Hello, world!";
		let key = "PBA";
		let _key = string_to_u8_16(key);
		let data = string_to_u8_16(plaintext);
		let cipher_text =  ecb_encrypt(data.to_vec(), _key);

        assert_eq!(
            vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 0, 0, 0],
            ecb_decrypt(cipher_text.to_vec(), _key)
        )
    }


	#[test]
    fn cbc_encrypt_test() {
		let plaintext = "Hello, world!";
		let key = "PBA";
		let _key = string_to_u8_16(key);
		let data = string_to_u8_16(plaintext);

        assert_eq!(
            vec![211, 31, 103, 243, 12, 56, 41, 155, 23, 60, 70, 227, 13, 165, 132, 46],
            cbc_encrypt(data.to_vec(), _key)
        )
    }

	#[test]
    fn cbc_decrypt_test() {
		let plaintext = "Hello, world!";
		let key = "PBA";
		let _key = string_to_u8_16(key);
		let data = string_to_u8_16(plaintext);
		let cipher_text = cbc_encrypt(data.to_vec(), _key);

        assert_eq!(
			vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 0, 0, 0],
            cbc_decrypt(cipher_text, _key)
        )
    }
}
