use crate::directions::{
    error::Error,
    request::Request,
    // Response is aliased to avoid collision with Isahc namespace.
    response::Response as DirectionsResponse,
}; // use

impl Request {

    /// Performs the HTTP get request and returns the response to the caller.
    ///
    /// # Arguments:
    ///
    /// This method accepts no arguments.

    pub fn get(&self) -> Result<DirectionsResponse, Error> {

        // Build the URL stem for the HTTP get request:

        const SERVICE_URL: &str = "https://maps.googleapis.com/maps/api/directions";
        const OUTPUT_FORMAT: &str = "json"; // json or xml
        let mut uri = format!("{}/{}?", SERVICE_URL, OUTPUT_FORMAT);

        match &self.query {
            // If query string built, append it to the URL stem.
            Some(query) => uri.push_str(query.as_ref()),
            // If query string not built, return an error.
            None => return Err(Error::QueryNotBuilt),
        } // match

        // Query the Google Cloud Maps Platform using using an HTTP get request,
        // and return result to caller:

        // let response = isahc::get(uri)?.json::<DirectionsResponse>()?;
        println!("{}", uri);
        let response = reqwest::blocking::get(&*uri)?.json::<DirectionsResponse>()?;
        Ok(response)

    } // fn

} // impl