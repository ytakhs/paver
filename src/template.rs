use crate::Result;
use serde::Deserialize;
use serde_yaml::Value;
use std::collections::HashMap;
use tera::{Context, Tera};

#[derive(Deserialize, Debug)]
struct Job {
    pub steps: Vec<HashMap<String, Value>>,
}

#[derive(Deserialize, Debug)]
pub struct Template {
    pub jobs: HashMap<String, Job>,
}

pub fn parse_template(filepath: impl AsRef<str>) -> Result<Template> {
    let tera = Tera::default();
    tera.add_template_file(filepath.as_ref(), None)?;
    let content = tera.render(filepath.as_ref(), &Context::new())?;

    let mut value: Template = serde_yaml::from_str(content.as_str())?;

    Ok(value)
}
