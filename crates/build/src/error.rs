use std::{
    env::VarError as StdEnvVarError, io::Error as StdIoError,
    path::StripPrefixError as StdPathStripPrefixError,
};

use tokio::task::JoinError as TokioTaskJoinError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    StdIoError(#[from] StdIoError),

    #[error(transparent)]
    StdPathStripPrefixError(#[from] StdPathStripPrefixError),

    #[error(transparent)]
    StdEnvVarError(#[from] StdEnvVarError),

    #[error(transparent)]
    TokioTaskJoinError(#[from] TokioTaskJoinError),

    #[error("{0}")]
    Generic(String),
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        Err($crate::error::Error::Generic(format!($($arg)*)))
    }
}

#[allow(unused_imports)]
pub(crate) use err;

pub type Result<T, E = Error> = std::result::Result<T, E>;
