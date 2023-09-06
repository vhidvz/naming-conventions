use crate::{tool, Convention};

use regex::{Captures, Error, Regex};

pub struct NoCase;

impl Convention for NoCase {
    fn to(&self, string: &str) -> Result<String, Error> {
        to_no_case(string)
    }

    fn is(&self, string: &str) -> Result<bool, Error> {
        is_no_case(string)
    }
}

pub fn to_no_case(string: &str) -> Result<String, Error> {
    let replacement =
        |haystack: &str, caps: &Captures, push: &Option<bool>| -> Result<String, Error> {
            let m = caps.get(0).unwrap();

            let mut term = String::from(" ");
            if push.unwrap() {
                term.push_str(&haystack[m.start()..m.end()]);
            }

            Ok(term)
        };

    let re = Regex::new(r"([A-Z][a-z]+)|([A-Z][A-Z]+)").unwrap();
    let result = tool::replace_all(&re, string, replacement, &Some(true)).unwrap();

    let re = Regex::new(r"(\s|[_-])+").unwrap();
    let result: String = tool::replace_all(&re, &result, replacement, &Some(false)).unwrap();

    log::debug!(target: "convention::no_case::to_no_case", "'{}' changed to '{}' (no case).", string, result.trim());
    Ok(result.trim().into())
}

pub fn is_no_case(string: &str) -> Result<bool, Error> {
    let no_case = to_no_case(string)?;
    Ok(no_case == string)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        dotenv::dotenv().ok();
        let _ = env_logger::try_init();
    }

    #[test]
    fn test_to_no_case() {
        tests::init();

        let result = to_no_case("vahid_Vakili ").unwrap();
        assert_eq!(&result, "vahid Vakili")
    }

    #[test]
    fn test_is_no_case() {
        tests::init();

        let result = is_no_case("vahid vakili").unwrap();
        assert_eq!(result, true);

        let result = is_no_case("vahid_vakili").unwrap();
        assert_eq!(result, false);
    }
}
