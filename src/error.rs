#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub enum DDApiError {
    Disconnected,
    Unsupported,
    AccessDenied,
    AccessLost,
    BadParam(String),
    Unexpected(String),
}
