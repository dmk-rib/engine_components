#[derive(Debug, Clone)]
pub struct XmlDocument {
    pub raw: String,
}

#[derive(Debug)]
pub struct XmlError {
    pub message: String,
}

impl std::fmt::Display for XmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for XmlError {}

pub struct XML;

impl XML {
    pub fn parse(xml: &str) -> Result<XmlDocument, XmlError> {
        Ok(XmlDocument {
            raw: xml.to_string(),
        })
    }

    pub fn build(document: &XmlDocument) -> Result<String, XmlError> {
        Ok(document.raw.clone())
    }
}
