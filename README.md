# this_actix_error

Example Usage

I recommend pairing this macro with [ThisError](https://github.com/dtolnay/thiserror)

```rust
#[derive(Error, Debug, ActixError)]
pub enum WebError {
    /// Has the Error Message Defined through ThisError, A Status code and a message used by ActixError
    #[status_code(500)]
    #[message("An Internal Error Occured")]
    #[error("Failed to read {0}")]
    Io(#[from] std::io::Error),
    /// Uses the Error Message Defined through ThisError, A Status code used by ActixError
    #[status_code(500)]
    #[error("Test Error")]
    Test,
    /// Same as the above but uses a Ident for the Status_code
    #[status_code(BAD_REQUEST)]
    #[error("Bad Request")]
    Two,
}
```