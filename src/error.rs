use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    BindFailed(IoError),
    ServerAlreadyStopped,
    ServerStopTimeout,
    ServerStopFailed,
}

impl Error {
    pub fn from_bind_error(bind_error: IoError) -> Self {
        Error::BindFailed(bind_error)
    }
}
