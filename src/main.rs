// Load the JSON config files (YAML?).
// State manager, input/output. Returns flag for whether to pass through the event or not.
// Basic tests with synthetic input/outputs.
// Call windows/macos from rust for synth events?

use std::path::Path;
mod context;
mod semantics;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = Path::new("src/user_config/contexts.txt");
    let mut contexts = context::parse_yaml_file(&config_path)?;
    
    let semantics_path = Path::new("src/user_config/semantics.txt");
    semantics::parse_semantics_file(&semantics_path, &mut contexts)?;
    
    for context in &contexts {
        println!("Context: {}", context.name);
        println!("  Aliases: {:?}", context.aliases);
        if let Some(parent) = &context.parent {
            println!("  Parent: {}", parent.name);
        }
        println!("  Semantic Actions:");
        for (action_name, action) in &context.semantic_actions {
            println!("    {}: {}", action_name, action.string_definition);
        }
        println!();
    }

    Ok(())
}