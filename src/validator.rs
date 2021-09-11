#[derive(Debug)]
pub struct ValidationResult {
    pub message: String
}

impl ValidationResult {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.into()
        }
    }
}

pub struct Validator {
    keywords: Vec<String>
}

impl Validator {
    pub fn new(keywords: Vec<String>) -> Self {
        Self {
            keywords: keywords
        }
    }

    fn primary_keyword(&self) -> Option<String> {
        self.keywords
            .first()
            .and_then(|keyword| Some(keyword.to_lowercase()))
    }

    fn validate_title(&self, website_title: &str) -> Vec<ValidationResult> {
        let downcased_title = website_title.to_lowercase();
        let mut results = vec!();
        if let Some(keyword) = self.primary_keyword() {
            if !downcased_title.contains(&keyword) {
                results.push(ValidationResult::new(&format!("Expected keyword '{}' in website title. Found: None", keyword)));
            }
        }
        results
    }

    pub fn validate(&self, website_title: &str) -> Vec<ValidationResult> {
        let mut validation_errors = vec!();
        validation_errors.append(&mut self.validate_title(website_title));
        validation_errors
    }
}