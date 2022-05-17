use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LispError {
    error_type: ErrorType,
    error_msg: Option<String>,
}

impl LispError {
    pub fn invalid_type(exp: impl ToString, got: impl ToString) -> Self {
        Self {
            error_type: ErrorType::InvalidType {
                exp: exp.to_string(),
                got: got.to_string(),
            },
            error_msg: None,
        }
    }

    pub fn wrong_number_of_args<T: ToString>(fn_name: T, got: usize, wanted: usize) -> Self {
        Self {
            error_type: ErrorType::WrongNumberOfArgs {
                fn_name: fn_name.to_string(),
                arguments_got: got,
                arguments_required: wanted,
            },
            error_msg: None,
        }
    }

    pub fn msg<T: ToString>(mut self, msg: T) -> Self {
        self.error_msg = Some(msg.to_string());
        self
    }
}

impl Display for LispError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self.error_msg.clone() {
            Some(msg) => write!(f, "[ERROR]: {}: {}", self.error_type, msg),
            None => write!(f, "[ERROR]: {}", self.error_type),
        }
    }
}

impl Error for LispError {}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ErrorType {
    InvalidType {
        exp: String,
        got: String,
    },
    WrongNumberOfArgs {
        fn_name: String,
        arguments_got: usize,
        arguments_required: usize,
    },
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Self::InvalidType { exp, got } => write!(f, "Expected {} but got {}", exp, got),
            Self::WrongNumberOfArgs {
                fn_name,
                arguments_got,
                arguments_required,
            } => {
                write!(
                    f,
                    "Function {} wants {} argumtents, but got {}",
                    fn_name, arguments_got, arguments_required
                )
            }
        }
    }
}
