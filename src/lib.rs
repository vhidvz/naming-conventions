mod tool;

use regex::Error;

pub mod conventions;

pub trait Convention {
    fn to(&self, string: &str) -> Result<String, Error>;
    fn is(&self, string: &str) -> Result<bool, Error>;
}
