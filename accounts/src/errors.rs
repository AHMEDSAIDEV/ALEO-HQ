#[derive(Debug, Error)]
pub enum QRCodeError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),
}

impl From<qrcode::types::QrError> for QRCodeError {
    fn from(error: qrcode::types::QrError) -> Self {
        QRCodeError::Crate("qrcode", format!("{:?}", error))
    }
}

impl From<QRCodeError> for std::fmt::Error {
    fn from(error: QRCodeError) -> Self {
        panic!("{:?}", error)
    }
}

#[derive(Debug, Error)]
pub enum PrivateKeyError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),
}

impl From<snarkos_errors::objects::account::AccountError> for PrivateKeyError {
    fn from(error: snarkos_errors::objects::account::AccountError) -> Self {
        PrivateKeyError::Crate("snarkos_errors::objects::account", format!("{:?}", error))
    }
}

impl From<std::io::Error> for PrivateKeyError {
    fn from(error: std::io::Error) -> Self {
        PrivateKeyError::Crate("std::io", format!("{:?}", error))
    }
}
