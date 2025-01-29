// Duplicating error types for now
// Once https://github.com/mozilla/uniffi-rs/issues/2392 is resolved
// we can use external error types

/// An Error.
#[derive(Debug, thiserror::Error, uniffi::Object)]
#[error("{e:?}")]
#[uniffi::export(Debug)]
pub struct GossipError {
    e: anyhow::Error,
}

#[uniffi::export]
impl GossipError {
    pub fn message(&self) -> String {
        self.to_string()
    }
}

impl From<anyhow::Error> for GossipError {
    fn from(e: anyhow::Error) -> Self {
        Self { e }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq, uniffi::Error)]
pub enum CallbackError {
    #[error("Callback failed")]
    Error,
}

impl From<CallbackError> for GossipError {
    fn from(e: CallbackError) -> Self {
        GossipError {
            e: anyhow::anyhow!("{:?}", e),
        }
    }
}

impl From<anyhow::Error> for CallbackError {
    fn from(_e: anyhow::Error) -> Self {
        CallbackError::Error
    }
}

impl From<uniffi::UnexpectedUniFFICallbackError> for CallbackError {
    fn from(_: uniffi::UnexpectedUniFFICallbackError) -> Self {
        CallbackError::Error
    }
}
