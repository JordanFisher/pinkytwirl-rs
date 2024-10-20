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
    engine.debug_print();

    Ok(())
}