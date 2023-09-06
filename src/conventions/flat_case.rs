use crate::{conventions::to_no_case, Convention};

use regex::{Error, Regex};

pub struct FlatCase;

impl Convention for FlatCase {
    fn to(&self, string: &str) -> Result<String, Error> {
        to_flat_case(string)
    }

    fn is(&self, string: &str) -> Result<bool, Error> {
        is_flat_case(string)
    }
}

pub fn to_flat_case(string: &str) -> Result<String, Error> {
    let no_case = to_no_case(string)?.to_lowercase();

    let re = Regex::new(r"\s+").unwrap();
    let result = re.replace_all(&no_case, "").to_string();

    log::debug!(target: "convention::flat_case::to_flat_case", "'{}' changed to '{}' (flat_case).", string, result);
    Ok(result)
}

pub fn is_flat_case(string: &str) -> Result<bool, Error> {
    let flat_case = to_flat_case(string)?;
    Ok(flat_case == string)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        dotenv::dotenv().ok();
        let _ = env_logger::try_init();
    }

    #[test]
    fn test_to_flat_case() {
        tests::init();

        let result = to_flat_case("vahid_Vakili ").unwrap();
        assert_eq!(&result, "vahidvakili")
    }

    #[test]
    fn test_is_flat_case() {
        tests::init();

        let result = is_flat_case("vahidvakili").unwrap();
        assert_eq!(result, true);

        let result = is_flat_case("vahid_Vakili").unwrap();
        assert_eq!(result, false);
    }
}
