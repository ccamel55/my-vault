# My Vault 

Password and secrets manager connector service. 

The initial goal of this project was to create a service similar to 1password connect 
but for Bitwarden.

In the future I would also like to add support for 1password, Keeper and LastPass 
under the same service. 
- https://github.com/lastpass/lastpass-cli
- https://developer.1password.com/docs/sdks/
- https://docs.keeper.io/en/enterprise-guide/developer-tools

### Use case

The connector is designed to be hosted and used in a trusted communication network. 
The currently supported methods being; private network using TCP or localhost using
unix sockets. 

**IT IS EXTREMELY IMPORTANT THAT THIS SERVICE IS NOT EXPOSED PUBLICLY**


### Why a connector service?

Any password or secret manager worth your data does client side encryption and uses the 
backend as a data store only. In layman's terms; the backend only stores encrypted data and
only the client can encrypt and decrypt information.

This has obvious security benefits but also means that forwarding requests directly to a 
service's backend is not possible without also implementing the appropriate client side
encryption scheme. 

### Existing implementation

Connectors exists for all the services mentioned however all have at least one of the
following limitations:
- support for only using one account at a time (not being able to access information across multiple accounts)
- requires some sort of human input on startup (can not be started without having the user log in etc.)
- rate limitation (data is not cached meaning normal service rate limitation exists)

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

## Architecture  

TODO


## References

- https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#pre-hashing-passwords
- https://contributing.bitwarden.com/architecture/sdk/
- https://github.com/bitwarden/clients/tree/main/apps/cli
- https://github.com/dani-garcia/vaultwarden/tree/main/src/api
- https://sdk-api-docs.bitwarden.com/bitwarden_core/index.html 

## Todo

- add tests for config
- add argon2 password hashing and tests
- update api to better represent MVC architecture and usage 
- add tests for tonic using mock https://github.com/hyperium/tonic/blob/master/examples/src/mock/mock.rs

- swap anyhow to defined errors
- client for testing
- integration tests

