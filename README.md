# AES MODES 
## Solution

Group 5
Members:
```
1. Abhiraj Mengade
2. Rohit Sarpotdar
3. Oliver
4. Kishan
```

This activity demonstrates the use of AES-128 block cipher in different modes of operation, including ECB, CBC, and CTR. These modes help in encrypting data of arbitrary length using a block cipher with a fixed-length input.

Features
- AES Encryption and Decryption: Basic AES-128 encryption and decryption for a single block.
- Padding and Unpadding: Properly pads data to fit block sizes and removes padding after decryption.
- Grouping and Ungrouping: Splits data into blocks of fixed size and combines them back.
- ECB Mode: Encrypts and decrypts data block by block without any additional security measures (not secure for real-world use).
- CBC Mode: Adds security by chaining blocks and using an initialization vector (IV).
- CTR Mode: Encrypts data by converting the block cipher into a stream cipher using a counter and nonce.

