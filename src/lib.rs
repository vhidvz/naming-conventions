mod tool;

use regex::{Captures, Error, Regex};
use tool::replace_all;

pub fn no_case(value: &str) -> Result<String, Error> {
    let replacement = |haystack: &str, caps: &Captures| -> Result<String, Error> {
        let m = caps.get(0).unwrap();

        let mut term = String::from(" ");
        term.push_str(&haystack[m.start()..m.end()]);

        Ok(term)
    };

    let haystack = value.clone();

    let re = Regex::new(r"[A-Z][a-z]+").unwrap();
    let result = replace_all(&re, haystack, replacement).unwrap();

    let re = Regex::new(r"[A-Z][A-Z]+").unwrap();
    let result = replace_all(&re, result.as_str(), replacement).unwrap();

    let re = Regex::new(r"[\s-_]+").unwrap();
    let result = replace_all(&re, result.as_str(), replacement).unwrap();

    Ok(result.trim().into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        dotenv::dotenv().ok();
        env_logger::builder().init();
    }

    #[test]
    fn test_no_case() {
        tests::init();

        let result = no_case("vahid_Vakili ");
    }
}
