use crate::config::validation_error::ConfigValidationError;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default)]
pub struct ConfigValidationReport {
    errors: Vec<ConfigValidationError>,
}

impl ConfigValidationReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_error(&mut self, error: ConfigValidationError) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn errors(&self) -> &[ConfigValidationError] {
        &self.errors
    }
}

impl Display for ConfigValidationReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Found {} config validation errors:", self.errors.len())?;
        for (i, error) in self.errors.iter().enumerate() {
            writeln!(f, "[{}] {}", i + 1, error)?;
        }
        Ok(())
    }
}
