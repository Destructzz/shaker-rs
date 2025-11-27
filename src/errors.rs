#[derive(thiserror::Error, Debug)]
pub enum ShakerError {
    #[error("Enable to fetch serial ports")]
    SerialsPorts(#[from] serialport::Error),
    #[error("No leonardo device")]
    NoLeonardo,
    #[error("Unable to write data")]
    IOWriting(#[from] std::io::Error),
    #[error("Serial port doesn't exist")]
    NoSerialPort,
    #[error("overflow bytes dx or dy variables for arduino input")]
    PositionBytesOverflow,
    #[error("overflow varible dx or dy for maximum")]
    PositionNormalOverflow,
}
