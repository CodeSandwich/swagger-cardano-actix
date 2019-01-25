mod error;
mod server_handler;

pub use self::error::Error;
pub use self::server_handler::ServerHandler;

pub type ServerResult<T> = Result<T, Error>;
