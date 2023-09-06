use crate::conventions::to_no_case;

use regex::{Error, Regex};

pub fn to_snake_case(string: &str) -> Result<String, Error> {
    let no_case = to_no_case(string)?.to_lowercase();

    let re = Regex::new(r"\s+").unwrap();
    let result = re.replace_all(&no_case, "_").to_string();

    log::debug!(target: "convention::snake_case::to_snake_case", "'{}' changed to '{}' (snake_case).", string, result);
    Ok(result)
}

pub fn is_snake_case(string: &str) -> Result<bool, Error> {
    let snake_case = to_snake_case(string)?;
    Ok(snake_case == string)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        dotenv::dotenv().ok();
        let _ = env_logger::try_init();
    }

    #[test]
    fn test_to_snake_case() {
        tests::init();

        let result = to_snake_case("vahid_Vakili ").unwrap();
        assert_eq!(&result, "vahid_vakili")
    }

    #[test]
    fn test_is_snake_case() {
        tests::init();

        let result = is_snake_case("vahid_vakili").unwrap();
        assert_eq!(result, true);

        let result = is_snake_case("vahid_Vakili").unwrap();
        assert_eq!(result, false);
    }
}
