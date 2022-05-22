use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Number(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{:?}", n),
        }
    }
}
