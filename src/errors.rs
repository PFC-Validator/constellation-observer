use thiserror::Error;

#[derive(Error, Debug)]
pub enum ObserverError {
    #[error("Socket Binary Message Unexpected")]
    SocketBinary,
    #[error("Socket Closed")]
    SocketClosed,
}
