use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppError(pub String);

impl<T: ToString> From<T> for AppError {
    fn from(value: T) -> Self {
        AppError(value.to_string())
    }
}
