use crate::{conventions::to_no_case, Convention};

use regex::{Error, Regex};

pub struct KebabCase;

impl Convention for KebabCase {
    fn to(&self, string: &str) -> Result<String, Error> {
        to_kebab_case(string)
    }

    fn is(&self, string: &str) -> Result<bool, Error> {
        is_kebab_case(string)
    }
}

pub fn to_kebab_case(string: &str) -> Result<String, Error> {
    let no_case = to_no_case(string)?.to_lowercase();

    let re = Regex::new(r"\s+").unwrap();
    let result = re.replace_all(&no_case, "-").to_string();

    log::debug!(target: "convention::kebab_case::to_kebab_case", "'{}' changed to '{}' (kebab_case).", string, result);
    Ok(result)
}

pub fn is_kebab_case(string: &str) -> Result<bool, Error> {
    let kebab_case = to_kebab_case(string)?;
    Ok(kebab_case == string)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        dotenv::dotenv().ok();
        let _ = env_logger::try_init();
    }

    #[test]
    fn test_to_kebab_case() {
        tests::init();

        let result = to_kebab_case("vahid_Vakili ").unwrap();
        assert_eq!(&result, "vahid-vakili")
    }

    #[test]
    fn test_is_kebab_case() {
        tests::init();

        let result = is_kebab_case("vahid-vakili").unwrap();
        assert_eq!(result, true);

        let result = is_kebab_case("vahid_Vakili").unwrap();
        assert_eq!(result, false);
    }
}
