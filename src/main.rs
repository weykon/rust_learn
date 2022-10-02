struct MyCustomError {
    code: i32,
    msg: String,
}

use std::fmt::{Debug, Display, Formatter};
impl std::error::Error for MyCustomError {}

impl Debug for MyCustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]{}", self.code, self.msg)
    }
}

impl Display for MyCustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}],{}", self.code, self.msg)
    }
}

fn main() -> Result<(), MyCustomError> {
    let one_error: Result<(), MyCustomError> = Err(MyCustomError {
        code: -1,
        msg: "error".to_owned(),
    });
    one_error?;

    Ok(())
}
