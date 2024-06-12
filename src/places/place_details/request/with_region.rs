use crate::places::place_details::request::Request;
use crate::types::Region;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {
    /// Adds the region parameter to the Places API _Place Details_ query.
    ///
    /// ## Arguments
    ///
    /// * `region` ‧ The region code, specified as a [ccTLD ("top-level
    ///   domain")](https://en.wikipedia.org/wiki/List_of_Internet_top-level_domains#Country_code_top-level_domains)
    ///   codes, two-character value. Most ccTLD codes are identical to ISO
    ///   3166-1 with some notable exceptions. For example, the United Kingdom's
    ///   ccTLD is "uk" (.co.uk) while its ISO 3166-1 code is "gb" (technically
    ///   for the entity of "The United Kingdom of Great Britain and Northern
    ///   Ireland").

    pub fn with_region(
        &'a mut self,
        region: impl Into<Region>
    ) -> &'a mut Self {
        // Set region in Request struct.
        self.region = Some(region.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl
