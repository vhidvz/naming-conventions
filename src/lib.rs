mod tool;

use regex::{Captures, Error, Regex};

pub fn no_case(haystack: &str) -> Result<String, Error> {
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
    let result = tool::replace_all(&re, haystack, replacement, &Some(true)).unwrap();

    let re = Regex::new(r"(\s|[_-])+").unwrap();
    let result = tool::replace_all(&re, &result, replacement, &Some(false)).unwrap();

    Ok(result.trim().into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        dotenv::dotenv().ok();
        let _ = env_logger::try_init();
    }

    #[test]
    fn test_no_case() {
        tests::init();

        let result = no_case("vahid_Vakili ").unwrap();
        assert_eq!(&result, "vahid Vakili")
    }
}
