#[derive(thiserror::Error, Debug)]
pub enum ShakerError {
    #[error("data store disconnected")]
    UnsupportedDevice,
    #[error("Enable to fetch serial ports")]
    SerialsPorts(#[from] serialport::Error),
    #[error("No leonardo device")]
    NoLeonardo,
    #[error("Unable to write data")]
    IOWriting(#[from] std::io::Error),
    #[error("Serial port doesn't exist")]
    NoSerialPort,
    #[error("overflow dx or dy variables")]
    PositionOverflow,
}
