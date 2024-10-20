use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct YamlContext {
    pub aliases: Vec<String>,
    pub parent: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SemanticAction {
    Sequence(Vec<SemanticAction>),
    Action(String),
    KeyCombination(Vec<String>),
    LiteralString(String),
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

pub fn parse_semantic_action(input: &str) -> SemanticAction {
    let parts: Vec<&str> = input.split('|').map(str::trim).collect();
    let mut sequence = Vec::new();

    for part in parts {
        if part.starts_with('"') && part.ends_with('"') {
            sequence.push(SemanticAction::LiteralString(part[1..part.len()-1].to_string()));
        } else if part.contains('+') {
            let keys: Vec<String> = part.split('+').map(|s| s.trim().to_string()).collect();
            sequence.push(SemanticAction::KeyCombination(keys));
        } else if part.contains('*') {
            let (count, action) = part.split_once('*').unwrap();
            let count: usize = count.trim().parse().unwrap_or(1);
            let action = action.trim().to_string();
            for _ in 0..count {
                sequence.push(SemanticAction::Action(action.clone()));
            }
        } else {
            sequence.push(SemanticAction::Action(part.to_string()));
        }
    }

    if sequence.len() == 1 {
        sequence.pop().unwrap()
    } else {
        SemanticAction::Sequence(sequence)
    }
}