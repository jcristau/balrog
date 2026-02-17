use crate::update::query::UpdateQuery;
use crate::error::AppError;

#[derive(Debug, PartialEq)]
pub enum ServeUpdate {
    Yes,
    No,
    Maybe, // Used when we need to check pinnable releases
}

pub trait Blob {
    fn schema_version(&self) -> i32;
    fn should_serve_update(&self, query: &UpdateQuery) -> Result<ServeUpdate, AppError>;
}

pub trait XmlBlob: Blob {
    fn get_header_xml(&self) -> String {
        "<?xml version=\"1.0\"?>".to_string()
    }

    fn get_inner_header_xml(&self, query: &UpdateQuery) -> Result<String, AppError>;
    fn get_inner_xml(&self, query: &UpdateQuery) -> Result<String, AppError>;

    fn get_inner_footer_xml(&self) -> String {
        String::new()
    }

    fn get_footer_xml(&self) -> String {
        "</updates>".to_string()
    }

    /// Generates the complete XML response
    fn get_xml(&self, query: &UpdateQuery) -> Result<String, AppError> {
        let mut lines = Vec::new();

        lines.push(self.get_header_xml());
        lines.push("<updates>".to_string());
        lines.push(self.get_inner_header_xml(query)?);
        lines.push(self.get_inner_xml(query)?);
        lines.push(self.get_inner_footer_xml());
        lines.push(self.get_footer_xml());

        Ok(lines.join("\n"))
    }
}
