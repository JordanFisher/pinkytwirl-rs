use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct YamlContext {
    pub aliases: Vec<String>,
    pub parent: Option<String>,
}

#[derive(Debug)]
pub struct Context {
    pub name: String,
    pub aliases: Vec<String>,
    pub parent: Option<Box<Context>>,
}

pub fn parse_yaml_file(file_path: &Path) -> Result<Vec<Context>, Box<dyn std::error::Error>> {
    let yaml_str = fs::read_to_string(file_path)?;
    Ok(parse_yaml(&yaml_str)?)
}

pub fn parse_yaml(yaml_str: &str) -> Result<Vec<Context>, serde_yaml::Error> {
    let yaml_contexts: HashMap<String, YamlContext> = serde_yaml::from_str(yaml_str)?;
    let mut contexts = HashMap::new();

    // First pass: Create Context objects without parents
    for (name, yaml_context) in &yaml_contexts {
        contexts.insert(name.clone(), Context {
            name: name.clone(),
            aliases: yaml_context.aliases.clone(),
            parent: None,
        });
    }

    // Second pass: Set parent relationships
    for (name, yaml_context) in &yaml_contexts {
        if let Some(parent_name) = &yaml_context.parent {
            if let Some(parent_context) = contexts.remove(parent_name) {
                if let Some(context) = contexts.get_mut(name) {
                    context.parent = Some(Box::new(parent_context));
                }
            }
        }
    }

    Ok(contexts.into_values().collect())
}