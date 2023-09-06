mod tool;

use regex::Error;

pub mod conventions;

pub use conventions::*;

pub enum CaseName {
    NoCase,
    SnakeCase,
    CamelCase,
    PascalCase,
    MacroCase,
    KebabCase,
    TrainCase,
    FlatCase,
}

pub trait Convention {
    fn to(&self, string: &str) -> Result<String, Error>;
    fn is(&self, string: &str) -> Result<bool, Error>;
}

/// The function `get_convention` returns a boxed trait object based on the given `CaseName` enum
/// variant.
///
/// Arguments:
///
/// * `name`: The `name` parameter is of type `CaseName`. It is an enum that represents different naming
/// conventions. The possible values for `CaseName` are:
///
/// Returns:
///
/// The function `get_convention` returns a boxed trait object that implements the `Convention` trait.
/// The specific implementation returned depends on the `CaseName` enum variant passed as an argument.
pub fn get_convention(name: CaseName) -> Box<dyn Convention> {
    match name {
        CaseName::NoCase => Box::new(NoCase),
        CaseName::SnakeCase => Box::new(SnakeCase),
        CaseName::CamelCase => Box::new(CamelCase),
        CaseName::PascalCase => Box::new(PascalCase),
        CaseName::MacroCase => Box::new(MacroCase),
        CaseName::KebabCase => Box::new(KebabCase),
        CaseName::FlatCase => Box::new(FlatCase),
        CaseName::TrainCase => Box::new(TrainCase),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        dotenv::dotenv().ok();
        let _ = env_logger::try_init();
    }

    #[test]
    fn test_get_convention() {
        tests::init();

        let no_case = get_convention(CaseName::NoCase);

        assert_eq!(no_case.to("hello, world!").unwrap(), "hello, world!")
    }
}
