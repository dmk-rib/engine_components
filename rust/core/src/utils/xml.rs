#[derive(Debug, Clone)]
pub struct XmlDocument {
    pub raw: String,
}

#[derive(Debug, Clone)]
pub struct XmlError {
    pub message: String,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct XmlParser;

impl XmlParser {
    pub fn parse(&self, input: &str) -> Result<XmlDocument, XmlError> {
        Ok(XmlDocument {
            raw: input.to_string(),
        })
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct XmlBuilder;

impl XmlBuilder {
    pub fn build(&self, document: &XmlDocument) -> Result<String, XmlError> {
        Ok(document.raw.clone())
    }
}

pub struct XML;

impl XML {
    pub fn parser() -> XmlParser {
        XmlParser::default()
    }

    pub fn builder() -> XmlBuilder {
        XmlBuilder::default()
    }
}
