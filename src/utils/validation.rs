use crate::routes::FormData;
use actix_web::web::Json;
use unicode_segmentation::UnicodeSegmentation;

pub fn validate_email(email: &str) -> bool {
    let re = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    re.is_match(email)
}

pub fn is_valid_name(s: &str) -> bool {
    let is_empty_or_whitespace = s.trim().is_empty();
    // A grapheme is defined by the Unicode standard as a "user-perceived"
    // character: `å` is a single grapheme, but it is composed of two characters // (`a` and `̊`).
    //
    // `graphemes` returns an iterator over the graphemes in the input `s`.
    // `true` specifies that we want to use the extended grapheme definition set,
    // the recommended one.
    let is_too_long = s.graphemes(true).count() > 256;
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
    // Return `false` if any of our conditions have been violated
    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}

#[allow(unused)]
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
