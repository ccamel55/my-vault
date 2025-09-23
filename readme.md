# bitwarden-cli-rs

## References

- https://contributing.bitwarden.com/architecture/sdk/
- https://github.com/bitwarden/clients/tree/main/apps/cli
- https://github.com/dani-garcia/vaultwarden/tree/main/src/api
- https://sdk-api-docs.bitwarden.com/bitwarden_core/index.html 

## Design - TODO

Goal: provide a background service which can interface with bitwarden. 

### Daemon  

Background process which manages the connection and state of local bitwarden vault. 

### CLI

CLI interface for daemon.