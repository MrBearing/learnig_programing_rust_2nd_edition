
use thiserror::Error;

#[derive(Error,Debug)]
#[error("{message:}({a:},{b:})")]
pub struct MySimpleError{
    message: String,
    a: String,
    b: String,
}


fn some_error_function() -> Result<(), MySimpleError> {
    Err(MySimpleError{
        message: "this is simple Error I defined using thiserror carate".to_string(),
        a: "a value".to_string(),
        b: "b value".to_string()
    })
}


fn main() -> Result<(), MySimpleError> {
    some_error_function()
}
