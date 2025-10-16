/// User table controller
pub struct ControllerUser;

impl shared_core::database::TableName for ControllerUser {
    const NAME: &'static str = "user";
}
