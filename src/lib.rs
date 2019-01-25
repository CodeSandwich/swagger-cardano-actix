mod error;
mod server_service;

pub use self::error::Error;
pub use self::server_service::ServerService;

pub type ServerResult<T> = Result<T, Error>;
