use crate::{conventions::to_no_case, tool, Convention};

use regex::{Captures, Error, Regex};

pub struct PascalCase;

impl Convention for PascalCase {
    fn to(&self, string: &str) -> Result<String, Error> {
        to_pascal_case(string)
    }

    fn is(&self, string: &str) -> Result<bool, Error> {
        is_pascal_case(string)
    }
}

pub fn to_pascal_case(string: &str) -> Result<String, Error> {
    let replacement =
        |haystack: &str, caps: &Captures, _options: &Option<bool>| -> Result<String, Error> {
            let m = caps.get(0).unwrap();
            Ok(haystack[m.start()..m.end()].to_uppercase())
        };

    let no_case = to_no_case(&string)?.to_lowercase();

    let mut haystack = String::from(" ");
    haystack.push_str(&no_case);

    let re = Regex::new(r"[^\w][a-z]").unwrap();
    let result = tool::replace_all(&re, &haystack, replacement, &None)?;

    let re = Regex::new(r"\s+").unwrap();
    let result = re.replace_all(&result, "").to_string();

    log::debug!(target: "convention::pascal_case::to_pascal_case", "'{}' changed to '{}' (pascal_case).", string, result);
    Ok(result)
}

pub fn is_pascal_case(string: &str) -> Result<bool, Error> {
    let pascal_case = to_pascal_case(string)?;
    Ok(pascal_case == string)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        dotenv::dotenv().ok();
        let _ = env_logger::try_init();
    }

    #[test]
    fn test_to_pascal_case() {
        tests::init();

        let result = to_pascal_case("vahid_Vakili ").unwrap();
        assert_eq!(&result, "VahidVakili")
    }

    #[test]
    fn test_is_pascal_case() {
        tests::init();

        let result = is_pascal_case("VahidVakili").unwrap();
        assert_eq!(result, true);

        let result = is_pascal_case("vahid_Vakili").unwrap();
        assert_eq!(result, false);
    }
}
