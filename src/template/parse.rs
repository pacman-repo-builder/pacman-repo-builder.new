/// Capability to parse a data structure into a template.
pub trait ParseTemplate {
    /// The parsed template.
    type Template<'a>
    where
        Self: 'a;
    /// Error when parsing fails.
    type Error;
    /// Parse the data structure into a template.
    fn parse_template(&self) -> Result<Self::Template<'_>, Self::Error>;
}
