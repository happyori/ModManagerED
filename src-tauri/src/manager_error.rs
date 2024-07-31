use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModManageredError {
    #[error(transparent)]
    Surreal(#[from] surrealdb::Error),
    #[error("ERR: {0}")]
    Generic(#[from] anyhow::Error),
    #[error("unknown error")]
    Unknown,
}

impl Serialize for ModManageredError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type ManagerResult<T> = Result<T, ModManageredError>;

#[macro_export]
macro_rules! generic {
     ($msg: literal) => {
         return Err($crate::manager_error::ModManageredError::from(anyhow!($msg)))
     };
}