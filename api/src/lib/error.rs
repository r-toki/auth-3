use serde_json::{json, Map as JsonMap, Value as JsonValue};
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unprocessable Entity: {0}")]
    UnprocessableEntity(JsonValue),
}

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Self {
        let mut err_map = JsonMap::new();

        for (field, errors) in errors.field_errors().iter() {
            let errors: Vec<JsonValue> = errors.iter().map(|error| json!(error.message)).collect();
            err_map.insert(field.to_string(), json!(errors));
        }

        Error::UnprocessableEntity(json!({ "errors": err_map }))
    }
}
