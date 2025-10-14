//! Get any `preprocessor.trpl-*` config.

use mdbook::preprocess::PreprocessorContext;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Default,
    Simple,
}

impl Mode {
    pub fn from_context(
        ctx: &PreprocessorContext,
        preprocessor_name: &str,
    ) -> Result<Mode, Error> {
        let config = ctx
            .config
            .get_preprocessor(preprocessor_name)
            .ok_or_else(|| Error::NoConfig(preprocessor_name.into()))?;

        let key = String::from("output-mode");
        let mode = config
            .get(&key)
            .map(|value| match value.as_str() {
                Some(s) => Mode::try_from(s).map_err(|_| Error::BadValue {
                    key,
                    value: value.to_string(),
                }),
                None => Err(Error::BadValue {
                    key,
                    value: value.to_string(),
                }),
            })
            .transpose()?
            .unwrap_or(Mode::Default);

        Ok(mode)
    }
}

/// Trivial marker struct to indicate an internal error.
///
/// The caller has enough info to do what it needs without passing data around.
pub struct ParseErr;

impl TryFrom<&str> for Mode {
    type Error = ParseErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "default" => Ok(Mode::Default),
            "simple" => Ok(Mode::Simple),
            _ => Err(ParseErr),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Mdbook(#[from] mdbook::errors::Error),

    #[error("No config for '{0}'")]
    NoConfig(String),

    #[error("Bad config value '{value}' for key '{key}'")]
    BadValue { key: String, value: String },
}

#[cfg(test)]
mod tests;
