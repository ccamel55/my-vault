# My Vault 

Password manager connector service. 

The initial goal of this project was to create a service similar to 1password connect 
but for Bitwarden. 

## Contributing

### Viewing local database

The daemon uses an `sqlcipher` database with encryption enabled to store sensitive information. 
In order to view the contents of the database we need the encryption key and a driver capable
of decrypting the database. 

The encryption key can be found in the config folder inside the `config.toml` file. 
 
A capable driver can be found [here](https://github.com/Willena/sqlite-jdbc-crypt/releases).

For Datagrip users, [this](https://intellij-support.jetbrains.com/hc/en-us/community/posts/360007633799-How-to-open-SQLCipher-passwrd-protected-file-in-Datagrip) 
thread has some helpful information on how to set up the driver.

- `jdbc:sqlite:{file:sqlite_db_file:}\?cipher=sqlcipher&legacy=4[&key={key:param:}]`

## Requirements

### Protobuf 

The daemon uses protobuf to send messages between itself and clients. Both the protobuf compiler
and Google common libs must be installed.

For Fedora protobuf can be installed with the following commands:

```shell
sudo dnf install protobuf protobuf-devel
```

For MacOS protobuf can be installed with the following commands:

```shell
brew install dbuf protobuf 
```

## Theory

TODO


## References

- https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#pre-hashing-passwords
- https://contributing.bitwarden.com/architecture/sdk/
- https://github.com/bitwarden/clients/tree/main/apps/cli
- https://github.com/dani-garcia/vaultwarden/tree/main/src/api
- https://sdk-api-docs.bitwarden.com/bitwarden_core/index.html 

## Todo

- jwt auth
- swap anyhow to defined errors
- hookup database
- client for testing
- integration tests
- remote database
- // https://github.com/hyperium/tonic/blob/master/examples/src/mock/mock.rs

