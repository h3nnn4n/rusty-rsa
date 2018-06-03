# Rusty RSA
Rusty RSA is yet another rust reimplementation of RSA. It was a bad name (which most likely I am not the first one to use) and should not be used for anything serious (other than getting my grades).

## Usage
--------

### Generating keys

```
rsa_lixo --keysize 256 --key totoro --generate_key
```

This will generate a 256 bits key named `totoro`. The files `totoro.pub` and `totoro.prv` will be generated. The keysize must be byte aligned, otherwise the read/write procedures will work incorrectly.

### Encrypting / Decrypting

```
rsa_lixo --keysize 256 --key totoro.pub --encrypt secret_file
```

This will use a key size of `256` stored in `totoro.pub` to encrypt the `secret_file`. The output will be placed at `secret_file.enc`. What happens if the private key is used here? Rusty RSA wont be able to decrypt it.
What happens if the key size is not the same as the one in the file? I have no idea. Things will break most likely.

```
rsa_lixo --keysize 256 --key totoro.prv --decrypt secret_file.enc
```

This decrypts the file. Works similar to `encrypt`.

### Breaking Keys

```
rsa_lixo --key novo.pub --METHOD
```

Where __method__ is one of the following: `bruteforce`, `pollardrho` or `fermats`. This will factor (eventually) the RSA modulus and generate the private key based on that. Pollard rho method can break keys up to 80 or 100
bits depending on the hardware being run.
