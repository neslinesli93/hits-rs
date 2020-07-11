use rusqlite::Error as RusqliteError;

pub type HitResult<T> = Result<T, HitError>;

#[derive(Debug)]
pub enum HitError {
    DatabaseError(RusqliteError),
    GenericError(String),
}

impl From<RusqliteError> for HitError {
    fn from(error: RusqliteError) -> Self {
        HitError::DatabaseError(error)
    }
}

impl From<String> for HitError {
    fn from(error: String) -> Self {
        HitError::GenericError(error)
    }
}
