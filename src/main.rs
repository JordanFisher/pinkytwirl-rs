// [x] Load the JSON config files (YAML?).
// [ ] Parse the inner semantics.
// [ ] Context lookup.
// [ ] State manager, input/output. Returns flag for whether to pass through the event or not.
// [ ] Basic tests with synthetic input/outputs.
// [ ] Embed into windows
// [ ] Embed into macOS

use std::path::Path;
mod contexts;
mod semantics;
mod mappings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = Path::new("src/user_config/contexts.txt");
    let mut contexts = contexts::parse_yaml_file(&config_path)?;
    
    let semantics_path = Path::new("src/user_config/semantics.txt");
    semantics::parse_semantics_file(&semantics_path, &mut contexts)?;
    
    let mappings_path = Path::new("src/user_config/mappings.txt");
    mappings::parse_mappings_file(&mappings_path, &mut contexts)?;
    
    for context in contexts.values() {
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

    Ok(())
}