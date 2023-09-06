use crate::conventions::to_no_case;

use regex::{Error, Regex};

pub fn to_macro_case(string: &str) -> Result<String, Error> {
    let no_case = to_no_case(string)?.to_uppercase();

    let re = Regex::new(r"\s+").unwrap();
    let result = re.replace_all(&no_case, "_").to_string();

    log::debug!(target: "convention::macro_case::to_macro_case", "'{}' changed to '{}' (macro_case).", string, result);
    Ok(result)
}

pub fn is_macro_case(string: &str) -> Result<bool, Error> {
    let macro_case = to_macro_case(string)?;
    Ok(macro_case == string)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        dotenv::dotenv().ok();
        let _ = env_logger::try_init();
    }

    #[test]
    fn test_to_macro_case() {
        tests::init();

        let result = to_macro_case("vahid_Vakili ").unwrap();
        assert_eq!(&result, "VAHID_VAKILI")
    }

    #[test]
    fn test_is_macro_case() {
        tests::init();

        let result = is_macro_case("VAHID_VAKILI").unwrap();
        assert_eq!(result, true);

        let result = is_macro_case("VAHID_Vakili").unwrap();
        assert_eq!(result, false);
    }
}
