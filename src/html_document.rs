use select::document::Document;
use select::predicate::Name;

pub struct HtmlDocument {
    document: Document
}

impl HtmlDocument {
    pub fn new(html_text: &str) -> Self {
        Self {
            document: Document::from(html_text)
        }
    }

    pub fn website_title(&self) -> Option<String> {
        let selects = &mut self.document.find(Name("title"));
        selects.nth(0)
                .and_then(|value| Some(value.text()))
                .or(None)
    }
}