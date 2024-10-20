// [x] Load the JSON config files (YAML?).
// [x] Parse the inner semantics.
// [ ] Context lookup.
// [ ] State manager, input/output. Returns flag for whether to pass through the event or not.
// [ ] Basic tests with synthetic input/outputs.
// [ ] Embed into windows
// [ ] Embed into macOS

// File: src/main.rs

use std::collections::HashMap;
use std::path::PathBuf;
use std::error::Error;
mod contexts;
mod semantics;
mod mappings;

pub struct PinkyTwirlEngine {
    contexts: HashMap<String, contexts::Context>,
    config_dir: PathBuf,
}

impl PinkyTwirlEngine {
    pub fn new(config_dir: PathBuf) -> Self {
        PinkyTwirlEngine {
            contexts: HashMap::new(),
            config_dir,
        }
    }

    pub fn load_configurations(&mut self) -> Result<(), Box<dyn Error>> {
        let contexts_path = self.config_dir.join("contexts.txt");
        self.contexts = contexts::parse_yaml_file(&contexts_path)?;

        let semantics_path = self.config_dir.join("semantics.txt");
        semantics::parse_semantics_file(&semantics_path, &mut self.contexts)?;

        let mappings_path = self.config_dir.join("mappings.txt");
        mappings::parse_mappings_file(&mappings_path, &mut self.contexts)?;

        Ok(())
    }

    pub fn get_context(&self, app_name: &str, window_name: &str) -> Option<&contexts::Context> {
        // Helper function for exact match
        let exact_match = |name: &str| -> Option<&contexts::Context> {
            self.contexts.values().find(|c| c.aliases.contains(&name.to_string()))
        };

        // Helper function for case-insensitive match
        let case_insensitive_match = |name: &str| -> Option<&contexts::Context> {
            let lower_name = name.to_lowercase();
            self.contexts.values().find(|c| 
                c.aliases.iter().any(|alias| alias.to_lowercase() == lower_name)
            )
        };

        // Helper function for substring match
        let substring_match = |name: &str| -> Option<&contexts::Context> {
            let lower_name = name.to_lowercase();
            self.contexts.values().find(|c| 
                c.aliases.iter().any(|alias| {
                    let lower_alias = alias.to_lowercase();
                    lower_alias.contains(&lower_name) || lower_name.contains(&lower_alias)
                })
            )
        };

        // Try exact match with app name
        exact_match(app_name)
            // Then try exact match with window name
            .or_else(|| exact_match(window_name))
            // Then try case-insensitive match with app name
            .or_else(|| case_insensitive_match(app_name))
            // Then try case-insensitive match with window name
            .or_else(|| case_insensitive_match(window_name))
            // Then try substring match with app name
            .or_else(|| substring_match(app_name))
            // Finally, try substring match with window name
            .or_else(|| substring_match(window_name))
    }

    pub fn debug_print(&self) {
        for context in self.contexts.values() {
            println!("Context: {}", context.name);
            println!("  Aliases: {:?}", context.aliases);
            println!("  Parent: {:?}", context.parent);
            println!("  Semantic Actions:");
            for (action_name, action) in &context.semantic_actions {
                println!("    {}: {:?}", action_name, action);
            }
            println!("  Key Mappings:");
            for (key, action) in &context.key_mappings {
                println!("    {}: {:?}", key, action);
            }
            println!();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_dir = PathBuf::from("src/user_config");
    let mut engine = PinkyTwirlEngine::new(config_dir);

    engine.load_configurations()?;
    // engine.debug_print();

    // Example usage of get_context
    let test_cases = vec![
        ("Visual Studio Code", "main.rs - MyProject"),
        ("FIREFOX", "Google - Mozilla Firefox"),
        ("cmd", "Command Prompt"),
        ("notepad++", "config.txt - Notepad++"),
        ("unknown_app", "Unknown Window"),
    ];

    for (app_name, window_name) in test_cases {
        match engine.get_context(app_name, window_name) {
            Some(context) => println!("Matched context for '{}' - '{}': {}", app_name, window_name, context.name),
            None => println!("No matching context found for '{}' - '{}'", app_name, window_name),
        }
    }
    
    Ok(())
}