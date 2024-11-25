# Overview


Keyshare is a binary for encoding data and splitting it into `n` shares. The `cipher_iv` file can be completely public, since you need the shares that are produced, to be able to decode it. It uses [ChaCha20Poly1305](https://docs.rs/chacha20poly1305/latest/chacha20poly1305/) to encode it, then splits it using [XOR secret sharing](https://wikipedia.org/wiki/Secret_sharing#t_=_n). These shares can then be given to other people, who store them securely for when you wish to access them next.

Note that with *(n,n) secret sharing*, which our [XOR secret sharing](https://wikipedia.org/wiki/Secret_sharing#t_=_n) is, if a single share is lost, it is unable to be recovered.

Also note that if even a single share is changed or mistyped, the output will error without returning the incorrect input.


# Example

>The first input in both cases will be either `encode` or `decode`, which signals to the code which function to use

## Encode

Encode takes:
1. The file path you want as input (`./hello.txt`),
2. The directory where you want to put the encoded files as a path (`./testing`),
3. The number of shares (`9`),
4. What you want to name the share files (`test`)

```shell
$ keyshare.exe encode ./hello.txt ./testing 9 test
```

which puts a [Serde](https://serde.rs/)-created JSON file called `cipher_iv` in the directory specified, as well as a set of `9` key shares labeled `test0` to `test8`

Ex:

- testing

    - cipher_iv
    - test0
    - test1
    - ...
    - test8



## Decode

Decode takes:
 1. The file path for the directory (`./testing`),
 2. The name you used for the files (`test`),
 3. The number of files (`9`),
 4. Where you want the output to go (`./decrypted_output.txt`),

 then reassembles the key. Then it uses that key and the `cipher_iv` file, which contains the initialization vector and the ciphertext, to reconstruct the plaintext.

```shell
$ keyshare.exe decode ./testing test 9 ./decrypted_output.txt
```

This results in `decrypted_output.txt` containing the original plaintext.
