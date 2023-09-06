use regex::{Captures, Error, Regex};

pub fn replace_all<ReplacementOptions>(
    re: &Regex,
    haystack: &str,
    replacement: impl Fn(&str, &Captures, &Option<ReplacementOptions>) -> Result<String, Error>,
    options: &Option<ReplacementOptions>,
) -> Result<String, Error> {
    let mut new = String::with_capacity(haystack.len());

    let mut last_match = 0;
    for caps in re.captures_iter(haystack) {
        let m = caps.get(0).unwrap();

        new.push_str(&haystack[last_match..m.start()]);
        new.push_str(&replacement(haystack, &caps, options)?);

        last_match = m.end();
    }

    new.push_str(&haystack[last_match..]);
    log::debug!(target: "tool::replace_all","'{haystack}' changed to '{new}' by regex replacement.");
    Ok(new)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        dotenv::dotenv().ok();
        let _ = env_logger::try_init();
    }

    #[test]
    fn test_replace_all() {
        tests::init();

        let haystack = "hello, world!";
        let re = Regex::new(r"world").unwrap();
        let replacement = |_haystack: &str,
                           _caps: &Captures,
                           _options: &Option<bool>|
         -> Result<String, Error> { Ok("vahid".into()) };

        let result = replace_all(&re, haystack, replacement, &None).unwrap();

        assert_eq!("hello, vahid!", result.to_string())
    }
}
