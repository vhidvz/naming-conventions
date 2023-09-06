use crate::{conventions::to_no_case, tool};

use regex::{Captures, Error, Regex};

pub fn to_camel_case(string: &str) -> Result<String, Error> {
    let replacement =
        |haystack: &str, caps: &Captures, _options: &Option<bool>| -> Result<String, Error> {
            let m = caps.get(0).unwrap();
            Ok(haystack[m.start()..m.end()].to_uppercase())
        };

    let no_case = to_no_case(string)?.to_lowercase();

    let re = Regex::new(r"[^\w][a-z]").unwrap();
    let result = tool::replace_all(&re, &no_case, replacement, &None)?;

    let re = Regex::new(r"\s+").unwrap();
    let result = re.replace_all(&result, "").to_string();

    log::debug!(target: "convention::camel_case::to_camel_case", "'{}' changed to '{}' (camel_case).", string, result);
    Ok(result)
}

pub fn is_camel_case(string: &str) -> Result<bool, Error> {
    let camel_case = to_camel_case(string)?;
    Ok(camel_case == string)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        dotenv::dotenv().ok();
        let _ = env_logger::try_init();
    }

    #[test]
    fn test_to_camel_case() {
        tests::init();

        let result = to_camel_case("vahid_Vakili ").unwrap();
        assert_eq!(&result, "vahidVakili")
    }

    #[test]
    fn test_is_camel_case() {
        tests::init();

        let result = is_camel_case("vahidVakili").unwrap();
        assert_eq!(result, true);

        let result = is_camel_case("VahidVakili").unwrap();
        assert_eq!(result, false);
    }
}
