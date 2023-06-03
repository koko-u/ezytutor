#[derive(Debug, derive_more::Display)]
#[display(fmt = "Application Error")]
pub struct AppError;

impl error_stack::Context for AppError {}
