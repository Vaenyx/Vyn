use std::{io, path::PathBuf};

use crate::tokenizing::Token;

#[derive(Debug, thiserror::Error)]
pub enum TokenizerError {
    #[error(transparent)]
    UnknownToken(#[from] UnknownTokenError),
}

#[derive(Debug, thiserror::Error)]
#[error("Found unknown token '{token:?}'")]
pub struct UnknownTokenError {
    pub token: Token,
}
