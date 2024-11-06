# Overview

Keyshare is a binary for akjdflkbvdlbvd


# Example
Encode takes the file you want to encode(`hello.txt`) as a path, the number of shares, and what you want to name the output file.

```shell
$ keyshare.exe encode ./hello.txt 9 test
```

Which creates a directory called `test` and puts a serde-created JSON file called `cipher_iv` in it as well as a set of `9` key shares labeled `test0` to `test8`


Decode takes the name you used to create the directory(`test`) as well as the number of files(`9`) and reassembles the key. Then it uses that key and the `cipher_iv` file, which contains the initialization vector and the ciphertext, to reconstruct the plaintext.

```shell
$ keyshare.exe decode test 9
```
