use inquire::error::InquireResult;

/// Use text prompt if value is none
pub fn unwrap_or_prompt(prompt: &str, value: Option<String>) -> InquireResult<String> {
    Ok(if let Some(value) = value {
        value
    } else {
        inquire::Text::new(prompt).prompt()?
    })
}
