use regex::{Captures, Error, Regex};

pub fn replace_all(
    re: &Regex,
    haystack: &str,
    replacement: impl Fn(&str, &Captures) -> Result<String, Error>,
) -> Result<String, Error> {
    let mut new = String::with_capacity(haystack.len());

    let mut last_match = 0;
    for caps in re.captures_iter(haystack) {
        let m = caps.get(0).unwrap();

        new.push_str(&haystack[last_match..m.start()]);
        new.push_str(&replacement(&haystack, &caps)?);

        last_match = m.end();
    }

    new.push_str(&haystack[last_match..]);
    Ok(new)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        dotenv::dotenv().ok();
        env_logger::builder().init();
    }

    #[test]
    fn test_replace_all() {
        tests::init();

        let haystack = "hello, world!";
        let re = Regex::new(r"world").unwrap();
        let replacement =
            |_haystack: &str, _caps: &Captures| -> Result<String, Error> { Ok("vahid".into()) };

        let result = replace_all(&re, haystack, &replacement).unwrap();

        assert_eq!("hello, vahid!", result.to_string())
    }
}
