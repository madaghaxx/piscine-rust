#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: &str, field_value: String, err: &str) -> Self {
        FormError {
            form_values: (field_name.to_string(), field_value),
            date: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err: err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn new(name: &str, password: &str) -> Self {
        Form {
            name: name.to_string(),
            password: password.to_string(),
        }
    }
    
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new("first_name", self.name.clone(), "Username is empty"));
        }
        
        if self.password.len() < 8 {
            return Err(FormError::new("password", self.password.clone(), "Password should be at least 8 characters long"));
        }
        
        let mut  chars = self.password.chars();
        let has_letter = chars.any(|c| c.is_ascii_alphabetic());
        let has_digit = chars.any(|c| c.is_ascii_digit());
        let has_symbol = chars.any(|c| !c.is_ascii_alphanumeric());
        
        if !has_letter || !has_digit || !has_symbol {
            return Err(FormError::new("password", self.password.clone(), "Password should be a combination of ASCII numbers, letters and symbols"));
        }
        
        Ok(())
    }
}
