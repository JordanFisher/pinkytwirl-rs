use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::contexts::{Context, SemanticAction};

pub fn parse_mappings_file(file_path: &Path, contexts: &mut HashMap<String, Context>) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut current_context: Option<&mut Context> = None;
    let mut current_prefix = String::new();

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
                println!("Warning: Context '{}' not found in mappings. Skipping.", context_name);
            }
            current_prefix.clear();
        } else if trimmed_line.ends_with(':') {
            // This is a prefix
            current_prefix = trimmed_line.trim_end_matches(':').to_string();
        } else if let Some(context) = &mut current_context {
            // This is a key mapping
            if let Some((key, value)) = trimmed_line.split_once('=') {
                let full_key = if current_prefix.is_empty() {
                    key.trim().to_string()
                } else {
                    format!("{} + {}", current_prefix, key.trim())
                };
                let action = value.trim().to_string();
                context.key_mappings.insert(full_key, SemanticAction {
                    string_definition: action,
                });
            }
        }
    }

    Ok(())
}