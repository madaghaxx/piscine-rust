#[derive(Debug, Eq, PartialEq)]
pub struct FormError<'a> {
    pub form_values: (&'a str, String),
    pub date: String,
    pub err: &'a str,
}

impl FormError <'_> {
    pub fn new(field_name: &str, field_value: String, err: &str) -> FormError<'static> {
        let now = chrono::Utc::now();
        let date = now.format("%Y-%m-%d %H:%M:%S").to_string();
        
        FormError {
            form_values: (field_name.to_string().leak(), field_value),
            date,
            err: err.to_string().leak(),
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

    pub fn validate(&self) -> Result<(), FormError<'_>> {
        if self.name.is_empty() {
            return Err(FormError::new(
                "name",
                self.name.clone(),
                "Username is empty",
            ));
        }

        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be at least 8 characters long",
            ));
        }
        let has_alpha = self.password.chars().any(|c| c.is_ascii_alphabetic());
        let has_numeric = self.password.chars().any(|c| c.is_ascii_digit());
        let has_symbol = self.password.chars().any(|c| !c.is_ascii_alphanumeric());

        if !(has_alpha && has_numeric && has_symbol) {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ));
        }

        Ok(())
    }
}