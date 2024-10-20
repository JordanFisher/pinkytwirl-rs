// Load the JSON config files (YAML?).
// State manager, input/output. Returns flag for whether to pass through the event or not.
// Call windows/macos from rust for synth events?

use std::path::Path;
mod context;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = Path::new("src/user_config/contexts.txt");
    let contexts = context::parse_yaml_file(&config_path)?;
    
    for context in &contexts {
        println!("Context: {:?}", context);
    }

    Ok(())
}