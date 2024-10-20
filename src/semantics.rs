use std::fs;
use std::path::Path;
use std::collections::HashMap;
use crate::contexts::{Context, SemanticAction, parse_semantic_action};

pub fn parse_semantics_file(file_path: &Path, contexts: &mut HashMap<String, Context>) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut current_context: Option<&mut Context> = None;

    for line in content.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with("//") {
            continue;
        }

        if !line.starts_with(" ") && !line.starts_with("\t") {
            // This is a context name
            let context_name = trimmed_line.trim_end_matches(':');
            current_context = contexts.get_mut(context_name);
            if current_context.is_none() {
                println!("Warning: Context '{}' not found. Skipping.", context_name);
            }
        } else if let Some(context) = &mut current_context {
            // This is a semantic action
            if let Some((key, value)) = trimmed_line.split_once('=') {
                let action_name = key.trim().to_string();
                let action_definition = value.trim().to_string();
                let semantic_action = parse_semantic_action(&action_definition);
                context.semantic_actions.insert(action_name, semantic_action);
            }
        }
    }

    Ok(())
}