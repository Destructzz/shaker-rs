use core::error;

#[derive(thiserror::Error, Debug)]
pub enum ShakerError {
    #[error("data store disconnected")]
    UnsupportedDevice,
    #[error("Enable to fetch serial ports")]
    SerialsPorts(#[from] serialport::Error),
    #[error("no leonardo device")]
    NoLeonardo,
}
