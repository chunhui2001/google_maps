use serde::{Serialize, Deserialize};

/// A representation of distance as a numeric value and a display string.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Distance {
    /// A string representation of the distance value, using the
    /// `with_unit_system()` specified in the request.
    text: String,
    /// Indicates the distance in meters.
    value: u32,
} // struct