use std::error::Error;

/* Validation to ensure that required fields have values */
pub fn err_if_none<T>(field: &Option<T>, fieldname: &str) -> Result<(), Box<dyn Error>> {
    match field {
        Some(_) => Ok(()),
        None => Err(format!("Field '{}' is empty, but is required", fieldname).into()),
    }
}

/* Validate a specific field to ensure it has a value, return it, otherwise return an error */
pub fn get_or_err<'a, T>(field: Option<&'a T>, fieldname: &str) -> Result<&'a T, Box<dyn Error>> {
    field.ok_or_else(|| format!("Field '{}' is empty, but is required", fieldname).into())
}
