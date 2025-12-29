# rust-crypt
A simple file protector made in rust that uses the kuznyechik encryption algorithm.

## Arguments
The arguments that can be used to run the encrypter/decrypter with.

| Argument | Description                                                                     |
|----------|---------------------------------------------------------------------------------|
| /verbose | If it should print out the files it has encrypted/decrypted.                    |
| /master  | The master key for the encryption/decryption.                                   |
| /folder  | The path to the folder whose contents will be encrypted/decrypted               |

## Technical Info
Uses Kuznyechik for encryption/decryption and Streebog512 for hashing.

loops through the folder <br>
generates a key for a file (100 characters) <br>
generates the suffix for the file (10 characters) <br>
derives the full key (file key + suffix + master key) <br>
reads the contents of the file and encrypts it <br>
creates a new file with the suffix and writes the encrypted data <br>
deletes the original file (blanks it first) <br>