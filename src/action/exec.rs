use crate::service::ServiceError;

pub trait Action {
    /// Executes the action.
    ///
    /// # Errors
    /// Returns `ServiceError` if the action fails to execute.
    fn execute(&self) -> Result<(), ServiceError>;
}
