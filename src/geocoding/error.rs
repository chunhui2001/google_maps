//! Resources (enums, traits) for the client to handle Geocoding API errors.

use crate::geocoding::response::Status;

/// Errors that may be produced by the Google Maps Geocoding API client.
#[derive(Debug)]
pub enum Error {
    /// Forward geocoding requests (address to latlng) must specify an `address`
    /// or at least one `component`.
    AddressOrComponentsRequired,
    /// Google Maps Geocoding API server generated an error. See the `Status`
    /// enum for more information.
    GoogleMapsGeocodingServer(Status, Option<String>),
    /// The dependency library Ihsahc generated an error.
    // Isahc(isahc::Error),
    /// The query string must be built before the request may be sent to the
    /// Google Maps Geocoding API server.
    QueryNotBuilt,
    /// The request must be validated before a query string may be built.
    RequestNotValidated,
    /// The dependency library Reqwest generated an error.
    Reqwest(reqwest::Error),
    /// The dependency library Serde JSON generated an error.
    SerdeJson(serde_json::error::Error),
} // enum

impl std::fmt::Display for Error {
    /// This trait converts the error code into a format that may be presented
    /// to the user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::AddressOrComponentsRequired => write!(f, "Google Maps Geocoding API client library: \
                Forward geocoding requests must specify an `address` or at least one `component`. \
                Ensure that the with_address() and/or with_component methods are being called before run()."),
            Error::GoogleMapsGeocodingServer(status, error_message) => match error_message {
                // If the Google Maps Geocoding API server generated an error
                // message, return that:
                Some(error_message) => write!(f, "Google Maps Geocoding API server: {}", error_message),
                // If the Google Maps Geocoding API server did not generate an
                // error message, return a generic message derived from the
                // response status:
                None => match status {
                    Status::InvalidRequest => write!(f, "Google Maps Geocoding API server: \
                        Invalid request. \
                        This may indicate that the query (address, components, or latlng) is missing, an invalid result type, or an invalid location type."),
                    Status::Ok => write!(f, "Google Maps Geocoding server: \
                        Ok. \
                        The request was successful."),
                    Status::OverDailyLimit => write!(f, "Google Maps Geocoding API server: \
                        Over daily limit. \
                        Usage cap has been exceeded, API key is invalid, billing has not been enabled, or method of payment is no longer valid."),
                    Status::OverQueryLimit => write!(f, "Google Maps Geocoding API server: \
                        Over query limit. \
                        Requestor has exceeded quota."),
                    Status::RequestDenied => write!(f, "Google Maps Geocoding API server: \
                        Request denied \
                        Service did not complete the request."),
                    Status::UnknownError => write!(f, "Google Maps Geocoding API server: \
                        Unknown error."),
                    Status::ZeroResults => write!(f, "Google Maps Geocoding API server: \
                        Zero results.
                        This may occur if the geocoder was passed a non-existent address."),
                } // match
            }, // match
            // Error::Isahc(error) => write!(f, "Google Maps Geocoding API client in the Isahc library: {}", error),
            Error::RequestNotValidated => write!(f, "Google Maps Geocoding API client library: \
                The request must be validated before a query string may be built. \
                Ensure the validate() method is called before build()."),
            Error::Reqwest(error) => write!(f, "Google Maps Geocoding API client in the Reqwest library: {}", error),
            Error::SerdeJson(error) => write!(f, "Google Maps Geocoding API client in the Serde JSON library: {}", error),
            Error::QueryNotBuilt => write!(f, "Google Maps Geocoding API client library: \
                The query string must be built before the request may be sent to the Google Cloud Maps Platform. \
                Ensure the build() method is called before run()."),
        } // match
    } // fn
} // impl

impl std::error::Error for Error {
    /// If the cause for the error is in an underlying library (not this
    /// library but a library this one depends on), this trait unwraps the
    /// original source error. This trait converts a Google Maps Geocoding API
    /// error type into the native error type of the underlying library.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::AddressOrComponentsRequired => None,
            Error::GoogleMapsGeocodingServer(_error, _message) => None,
            // Error::Isahc(error) => Some(error),
            Error::RequestNotValidated => None,
            Error::Reqwest(error) => Some(error),
            Error::SerdeJson(error) => Some(error),
            Error::QueryNotBuilt => None,
        } // match
    } // fn
} // impl

/* impl From<isahc::Error> for Error {
    /// This trait converts from an Isahc error type (`isahc::Error`) into a
    /// Google Maps Geocoding API error type
    /// (`google_maps::geocoding::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: isahc::Error) -> Error {
        Error::Isahc(error)
    } // fn
} // impl */

impl From<reqwest::Error> for Error {
    /// This trait converts from an Isahc error type (`reqwest::Error`) into a
    /// Google Maps Geocoding API error type
    /// (`google_maps::geocoding::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: reqwest::Error) -> Error {
        Error::Reqwest(error)
    } // fn
} // impl

impl From<serde_json::error::Error> for Error {
    /// This trait converts from an Serde JSON (`serde_json::error::Error`)
    /// error type into a Google Maps Geocoding API error type
    /// (`google_maps::geocoding::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: serde_json::error::Error) -> Error {
        Error::SerdeJson(error)
    } // fn
} // impl