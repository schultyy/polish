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

    pub fn validate(&self, website_title: &str) -> Vec<ValidationResult> {
        let mut validation_errors = vec!();
        let downcased_title = website_title.to_lowercase();

        if let Some(keyword) = self.primary_keyword() {
            if !downcased_title.contains(&keyword) {
                validation_errors.push(ValidationResult::new(&format!("Expected keyword '{}' in website title. Found: None", keyword)));
            }
        }

        validation_errors
    }
}