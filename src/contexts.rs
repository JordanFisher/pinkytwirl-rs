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
pub struct SemanticAction {
    pub string_definition: String,
}

#[derive(Debug)]
pub struct Context {
    pub name: String,
    pub aliases: Vec<String>,
    pub parent: Option<String>,
    pub semantic_actions: HashMap<String, SemanticAction>,
    pub key_mappings: HashMap<String, SemanticAction>,
}

pub fn parse_yaml_file(file_path: &Path) -> Result<HashMap<String, Context>, Box<dyn std::error::Error>> {
    let yaml_str = fs::read_to_string(file_path)?;
    Ok(parse_yaml(&yaml_str)?)
}

pub fn parse_yaml(yaml_str: &str) -> Result<HashMap<String, Context>, serde_yaml::Error> {
    let yaml_contexts: HashMap<String, YamlContext> = serde_yaml::from_str(yaml_str)?;
    let mut contexts = HashMap::new();

    // First pass: Create Context objects.
    for (name, yaml_context) in &yaml_contexts {
        println!("Creating context: {}", name);
        contexts.insert(name.clone(), Context {
            name: name.clone(),
            aliases: yaml_context.aliases.clone(),
            parent: yaml_context.parent.clone(),
            semantic_actions: HashMap::new(),
            key_mappings: HashMap::new(),
        });
    }

    // Second pass: Make sure that parent contexts exists if set.
    let context_names: Vec<String> = contexts.keys().cloned().collect();
    for context in contexts.values_mut() {
        if let Some(parent_name) = &context.parent {
            if !context_names.contains(parent_name) {
                println!("Warning: Parent context '{}' not found for context '{}'. Skipping.", parent_name, context.name);
                context.parent = None;
            }
        }
    }

    Ok(contexts)
}