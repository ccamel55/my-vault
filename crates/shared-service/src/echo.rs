#[tarpc::service]
pub trait Echo {
    /// Health check
    async fn health_check();

    /// Echo message back to sender
    async fn echo(message: String) -> String;
}
