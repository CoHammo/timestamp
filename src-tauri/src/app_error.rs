use serde::Serialize;

// #[derive(Debug, Serialize)]
// pub enum AppError {
//     Db(String),
//     TimeParse(String),
//     Generic(String),
// }

#[derive(Debug, Serialize)]
pub struct AppError(pub String);

// impl std::fmt::Display for AppError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "AppError({})", self.0)
//     }
// }

// impl Error for AppError {}

// impl ToString for AppError {
//     fn to_string(&self) -> String {
//         return self.0.clone();
//     }
// }

// impl std::fmt::Display for AppError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "Error: {}", self)
//     }
// }

// impl From<turso::Error> for AppError {
//     fn from(value: turso::Error) -> Self {
//         AppError::Db(value.to_string())
//     }
// }

// impl From<chrono::ParseError> for AppError {
//     fn from(value: turso::Error) -> Self {
//         AppError::TimeParse(value.to_string())
//     }
// }

impl<T: ToString> From<T> for AppError {
    fn from(value: T) -> Self {
        AppError(value.to_string())
    }
}
