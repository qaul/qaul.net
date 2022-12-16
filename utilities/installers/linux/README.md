## Running locally

### Setting up the environment
Before proceeding, make sure you're in the proper directory:
```shell
cd qaul_ui/snap
```

To run locally, you need to export the snapcraft credentials to allow the snapcraft command to authenticate without a password:
```shell
snapcraft export-login credentials
```

> This file is not encrypted by default, so don't add it to version control!

Next, we'll encrypt the output file `credentials`:
```shell
sudo apt install openssl
openssl aes-256-cbc -e -in credentials -out snap_credentials.enc

rm credentials

export SNAPCRAFT_CREDENTIALS_KEY=<The key used to encrypt the credentials file>
```

Now, we can run the make command:
```shell
cd ../..  # Go back to the root directory
echo $SNAPCRAFT_CREDENTIALS_KEY  # Make sure the encryption key is set

make bundle-snapfile
```
