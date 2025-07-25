use core::fmt;

use crate::ohttp::DirectoryResponseError;
use crate::uri::url_ext::ParseReceiverPubkeyParamError;

/// Error returned when request could not be created.
///
/// This error can currently only happen due to programmer mistake.
/// `unwrap()`ing it is thus considered OK in Rust but you may achieve nicer message by displaying
/// it.
#[derive(Debug)]
pub struct CreateRequestError(InternalCreateRequestError);

#[derive(Debug)]
pub(crate) enum InternalCreateRequestError {
    Url(crate::into_url::Error),
    Hpke(crate::hpke::HpkeError),
    OhttpEncapsulation(crate::ohttp::OhttpEncapsulationError),
    ParseReceiverPubkey(ParseReceiverPubkeyParamError),
    MissingOhttpConfig,
    Expired(std::time::SystemTime),
}

impl fmt::Display for CreateRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use InternalCreateRequestError::*;

        match &self.0 {
            Url(e) => write!(f, "cannot parse url: {e:#?}"),
            Hpke(e) => write!(f, "v2 error: {e}"),
            OhttpEncapsulation(e) => write!(f, "v2 error: {e}"),
            ParseReceiverPubkey(e) => write!(f, "cannot parse receiver public key: {e}"),
            MissingOhttpConfig =>
                write!(f, "no ohttp configuration with which to make a v2 request available"),
            Expired(expiry) => write!(f, "session expired at {expiry:?}"),
        }
    }
}

impl std::error::Error for CreateRequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use InternalCreateRequestError::*;

        match &self.0 {
            Url(error) => Some(error),
            Hpke(error) => Some(error),
            OhttpEncapsulation(error) => Some(error),
            ParseReceiverPubkey(error) => Some(error),
            MissingOhttpConfig => None,
            Expired(_) => None,
        }
    }
}

impl From<InternalCreateRequestError> for CreateRequestError {
    fn from(value: InternalCreateRequestError) -> Self { CreateRequestError(value) }
}

impl From<crate::into_url::Error> for CreateRequestError {
    fn from(value: crate::into_url::Error) -> Self {
        CreateRequestError(InternalCreateRequestError::Url(value))
    }
}

impl From<ParseReceiverPubkeyParamError> for CreateRequestError {
    fn from(value: ParseReceiverPubkeyParamError) -> Self {
        CreateRequestError(InternalCreateRequestError::ParseReceiverPubkey(value))
    }
}

/// Error returned for v2-specific payload encapsulation errors.
#[derive(Debug)]
pub struct EncapsulationError(InternalEncapsulationError);

#[derive(Debug)]
pub(crate) enum InternalEncapsulationError {
    /// The HPKE failed.
    Hpke(crate::hpke::HpkeError),
    /// The directory returned a bad response
    DirectoryResponse(DirectoryResponseError),
}

impl fmt::Display for EncapsulationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use InternalEncapsulationError::*;

        match &self.0 {
            Hpke(error) => write!(f, "HPKE error: {error}"),
            DirectoryResponse(e) => write!(f, "Directory response error: {e}"),
        }
    }
}

impl std::error::Error for EncapsulationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use InternalEncapsulationError::*;

        match &self.0 {
            Hpke(error) => Some(error),
            DirectoryResponse(e) => Some(e),
        }
    }
}

impl From<InternalEncapsulationError> for EncapsulationError {
    fn from(value: InternalEncapsulationError) -> Self { EncapsulationError(value) }
}

impl From<InternalEncapsulationError> for super::ResponseError {
    fn from(value: InternalEncapsulationError) -> Self {
        super::InternalValidationError::V2Encapsulation(value.into()).into()
    }
}
