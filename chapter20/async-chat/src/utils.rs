use std::error::Error;

// TODO anyhowの利用して実装してみる
pub type ChatError = Box<dyn Error+Send + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

