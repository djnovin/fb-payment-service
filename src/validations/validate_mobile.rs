use validator::ValidationError;

pub fn validate_australian_mobile_number(phone_number: &str) -> Result<(), ValidationError> {
    let re = regex::Regex::new(r"^\+?61\d{9}$").unwrap();
    if re.is_match(phone_number) {
        Ok(())
    } else {
        let mut error = ValidationError::new("invalid_australian_mobile_number");
        error.message = Some(format!("Invalid Australian mobile number {}", phone_number).into());
        Err(error)
    }
}
