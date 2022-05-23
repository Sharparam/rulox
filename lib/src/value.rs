use std::fmt::{self, Display, Formatter};

use crate::vm::RuntimeError;

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Number(f64),
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Number(value.into())
    }
}

impl TryFrom<Value> for f64 {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, RuntimeError> {
        match value {
            Value::Number(value) => Ok(value),
            // _ => Err(RuntimeError::TypeError),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{:?}", n),
        }
    }
}
