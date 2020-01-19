//! Resources (enums, structs, methods) for the client to build a Time Zone API
//! request.

mod build;
mod get;
mod new;
mod positional_request;
mod sampled_path_request;
mod validate;

use crate::latlng::LatLng;

/// Use this structure's methods to build a Time Zone API request.

#[derive(Clone, Debug)]
pub struct Request {

    // Required parameters:
    // --------------------

    /// Your application's API key. This key identifies your application for
    /// purposes of quota management. Learn how to
    /// [get a key](https://developers.google.com/maps/documentation/timezone/get-api-key).
    key: String,

    // Positional Requests:
    // --------------------

    /// Defines the location(s) on the earth from which to return elevation
    /// data. This parameter takes either a single location as a
    /// latitude/longitude pair, multiple latitude/longitude pairs, or an
    /// encoded polyline.
    locations: Option<Locations>,

    // Sampled Path Requests:
    // ----------------------

    /// Defines a path on the earth for which to return elevation data. This
    /// parameter defines a set of two or more ordered latitude/longitude
    /// pairs defining a path along the surface of the earth. This parameter
    /// must be used in conjunction with the `samples` parameter described
    /// below.
    path: Option<Locations>,

    /// Specifies the number of sample points along a path for which to return
    /// elevation data. The samples parameter divides the given path into an
    /// ordered set of equidistant points along the path.
    samples: Option<u8>,

    // Internal use only:
    // ------------------

    /// Has the request been validated?
    validated: bool,

    /// Query string that is to be submitted to the Google Cloud Maps Platform.
    query: Option<String>,

} // struct

/// Defines the location(s) on the earth from which to return elevation
/// data. This parameter takes either a single location as a latitude/longitude
/// pair, multiple latitude/longitude pairs, or an encoded polyline.

#[derive(Clone, Debug)]
pub enum Locations {

    /// A single or multiple [latitude/longitude](https://developers.google.com/maps/documentation/elevation/intro#Locations)
    /// pairs.
    LatLngs(Vec<LatLng>),

    /// An [encoded polyline](https://developers.google.com/maps/documentation/utilities/polylinealgorithm).
    Polyline(String),

} // enum

impl From<&Locations> for String {
    /// Converts a `Locations` enum to a `String` that contains
    /// [locations](https://developers.google.com/maps/documentation/elevation/intro#Usage).
    fn from(locations: &Locations) -> String {
        match locations {
            Locations::LatLngs(latlngs) => String::from(
                latlngs.iter().map(|latlng| String::from(latlng) + "|").collect::<String>().trim_end_matches('|')
            ),
            Locations::Polyline(polyline) => format!("enc:{}", polyline),
        } // match
    } // fn
} // impl