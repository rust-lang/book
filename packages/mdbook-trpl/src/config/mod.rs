//! Get any `preprocessor.trpl-*` config.

use mdbook_preprocessor::PreprocessorContext;

#[derive(Debug, Default, Clone, Copy, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    #[default]
    Default,
    Simple,
}

impl Mode {
    pub fn from_context(
        ctx: &PreprocessorContext,
        preprocessor_name: &str,
    ) -> Result<Mode, Error> {
        #[derive(Default, serde::Deserialize)]
        #[serde(rename_all = "kebab-case")]
        struct Config {
            #[serde(default)]
            output_mode: Mode,
        }

        let config: Config = ctx
            .config
            .get(&format!("preprocessor.{preprocessor_name}"))?
            .ok_or_else(|| Error::NoConfig(preprocessor_name.into()))?;
        Ok(config.output_mode)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Mdbook(#[from] mdbook_preprocessor::errors::Error),

    #[error("No config for '{0}'")]
    NoConfig(String),
}

#[cfg(test)]
mod tests;
