# rust-crypt
A simple file protector made in rust that uses the kuznyechik encryption algorithm. <br>
This should NOT be used to encrypt any important data as it uses the CTR cipher mode and has no checks. If you use the wrong key for decryption, the original data will probably be lost forever. <br>

## Arguments
The arguments that can be used to run the encrypter/decrypter with.

| Argument | Description                                                        |
|----------|--------------------------------------------------------------------|
| /verbose | If it should print out the files it has encrypted/decrypted.       |
| /master  | The master key for the encryption/decryption.                      |
| /folder  | The path to the folder whose contents will be encrypted/decrypted. |

### example:
```bash
.\decrypter.exe /verbose /master themasterkeuy /folder "C:\Users\user\Desktop\test folder"
```

## Showcase
https://www.youtube.com/watch?v=1HPKXYrCBqQ

[<img src="https://img.youtube.com/vi/1HPKXYrCBqQ/hqdefault.jpg" width="600" height="300"
/>](https://www.youtube.com/embed/1HPKXYrCBqQ)

## Technical Info
Uses Kuznyechik for encryption/decryption and Streebog512 for hashing.

Loops through the folder. <br>
Generates a key for a file (100 characters). <br>
Generates the suffix for the file (10 characters). <br>
Derives the full key (file key + suffix + master key). <br>
Reads the contents of the file and encrypts it. <br>
Creates a new file with the suffix and writes the encrypted data. <br>
Deletes the original file (blanks it first). <br>
