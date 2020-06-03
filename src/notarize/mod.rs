mod run;

use crate::util::input_path::PathType;
use std::error::Error;
use std::path::PathBuf;
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub enum Password {
    Keychain(String),
    Env(String),
    Literal(String),
}

impl Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Password::Keychain(p) => write!(f, "@keychain:{}", p),
            Password::Env(p) => write!(f, "@env:{}", p),
            Password::Literal(p) => write!(f, "{}", p),
        }
    }
}

#[derive(Debug)]
pub(crate) struct NotarizeOp {
    input_path: PathBuf,
    path_type: PathType,
    bundle_id: String,
    developer_account: String,
    password: Password,
    provider: Option<String>,
}

pub(crate) fn run(
    input_path: PathBuf,
    path_type: PathType,
    bundle_id: String,
    developer_account: String,
    password: Password,
    provider: Option<String>,
) -> Result<(), Box<dyn Error>> {
    NotarizeOp::new(
        input_path,
        path_type,
        bundle_id,
        developer_account,
        password,
        provider,
    )
    .run()
}
