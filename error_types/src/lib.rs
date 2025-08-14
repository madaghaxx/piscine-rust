#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> Self {
        let now = chrono::Utc::now();
        let date = now.format("%Y-%m-%d %H:%M:%S").to_string();
        
        FormError {
            form_values: (field_name, field_value),
            date,
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
    pub fn new(name: String, password: String) -> Self {
        Form { name, password }
    }

    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new(
                "first_name".to_string(),
                self.name.clone(),
                "Username is empty".to_string(),
            ));
        }

        if self.password.len() < 8 {
            return Err(FormError::new(
                "password".to_string(),
                self.password.clone(),
                "Password should be at least 8 characters long".to_string(),
            ));
        }
        let has_alpha = self.password.chars().any(|c| c.is_ascii_alphabetic());
        let has_numeric = self.password.chars().any(|c| c.is_ascii_digit());
        let has_symbol = self.password.chars().any(|c| !c.is_ascii_alphanumeric());

        if !(has_alpha && has_numeric && has_symbol) {
            return Err(FormError::new(
                "password".to_string(),
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols".to_string(),
            ));
        }

        Ok(())
    }
}