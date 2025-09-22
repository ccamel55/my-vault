#[tarpc::service]
pub trait Echo {
    /// Health Check
    async fn health_check();

    /// Echo "hello {name}"
    async fn echo(name: String) -> String;
}
