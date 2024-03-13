use actix_web::web::Json;

use crate::routes::FormData;

pub fn validate_email(email: &str) -> bool {
    let re = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    re.is_match(email)
}

pub fn validate_form_data(data: Json<FormData>) -> Result<Json<FormData>, String> {
    let mut error_msg = String::new();
    if data.name.is_empty() && data.email.is_empty() {
        error_msg = "missing both name and email".to_string();
    } else if data.name.is_empty() {
        error_msg = "missing the name".to_string();
    } else if data.email.is_empty() {
        error_msg = "missing the email".to_string();
    } else if !validate_email(&data.email) {
        error_msg = "has an invalid email".to_string();
    }
    if !error_msg.is_empty() {
        return Err(error_msg);
    }
    Ok(data)
}
