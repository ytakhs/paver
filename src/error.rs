use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Action error: {0}")]
    ActionError(String),

    #[error("Parse template error: {source}")]
    ParseTemplateError {
        #[from]
        source: tera::Error,
    },
    #[error("YAML parse error: #{source}")]
    YamlParseError {
        #[from]
        source: serde_yaml::Error,
    },

    #[error("IO error: #{source}")]
    IOError {
        #[from]
        source: std::io::Error,
    },

    #[error("UTF8 error: #{source}")]
    Utf8Error {
        #[from]
        source: std::str::Utf8Error,
    },
}
