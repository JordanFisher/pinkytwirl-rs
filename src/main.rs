// [x] Load the JSON config files (YAML?).
// [x] Parse the inner semantics.
// [x] Context lookup.
// [ ] KeyCombination(Vec<String>) should become Key(Key), which is just a key with boolean meta
    // include the key code?
// [ ] State manager, input/output. Returns flag for whether to pass through the event or not.
    // copy from claude, but return a Vec<Key> instead of Vec<String>
// [ ] Basic tests with synthetic input/outputs.
// [ ] Embed into windows
// [ ] Embed into macOS

use std::path::{Path, PathBuf};
use std::error::Error;
use std::collections::{HashMap, VecDeque};

use contexts::{key_down, key_press, key_up, KeyEvent};
mod contexts;
mod semantics;
mod mappings;

pub struct PinkyTwirlEngine {
    contexts: HashMap<String, contexts::Context>,
    config_dir: PathBuf,
    pressed_keys: VecDeque<KeyEvent>,
    current_context: Option<String>,
}

impl PinkyTwirlEngine {
    pub fn new(config_dir: PathBuf) -> Self {
        PinkyTwirlEngine {
            contexts: HashMap::new(),
            config_dir,
            pressed_keys: VecDeque::new(),
            current_context: None,
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

    pub fn print_config(&self) {
        for context in self.contexts.values() {
            println!("Context: {}", context.name);
            println!("  Aliases: {:?}", context.aliases);
            println!("  Parent: {:?}", context.parent);
            println!("  Semantic Actions:");
            for (action_name, action) in &context.semantic_actions {
                println!("    {}: {}", action_name, action);
            }
            println!("  Key Mappings:");
            for (key, action) in &context.key_mappings {
                println!("    {}: {}", key, action);
            }
            println!();
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent, app_name: &str, window_name: &str) -> Vec<KeyEvent> {
        let mut synthetic_events = Vec::new();

        match event.state {
            contexts::KeyState::Down => {
                self.pressed_keys.push_back(event.clone());
                
                if let Some(context) = self.get_context(app_name, window_name) {
                    dbg!(&self.pressed_keys);
                    dbg!(&context.name);
                    if let Some(action) = self.find_chord_action(&context, &self.pressed_keys) {
                        synthetic_events = self.resolve_semantic_action(&action, context);
                        self.pressed_keys.clear();
                    } else if self.pressed_keys.len() == 1 {
                        // If it's the first key and doesn't match any chord, let it through
                        return Vec::new();
                    } else {
                        // If it doesn't match any chord, play back the buffered keys
                        synthetic_events = self.pressed_keys.iter().cloned().collect();
                        self.pressed_keys.clear();
                    }
                } else {
                    // If no context is found, let the key through
                    return Vec::new();
                }
            }
            contexts::KeyState::Up => {
                self.pressed_keys.retain(|k| &k.key != &event.key);
                if self.pressed_keys.is_empty() {
                    self.current_context = None;
                }
            }
            contexts::KeyState::DownUp => {}
        }

        synthetic_events
    }

    fn find_chord_action<'a>(&self, context: &'a contexts::Context, chord: &VecDeque<KeyEvent>) -> Option<contexts::SemanticAction> {
        let chord_str = chord.iter().map(|key| key.key.clone()).collect::<Vec<String>>().join(" + ");
        dbg!(&chord_str);
        // Lookup the chord in the context's key mappings. If it's not found, try the parent context, then the parent parent, etc.
        let mut current_context = Some(context);
        while let Some(context) = current_context {
            if let Some(action) = context.key_mappings.get(&chord_str) {
                println!("Found action: {}", action);
                return Some(action.clone());
            }
            current_context = context.parent.as_ref().and_then(|parent| self.contexts.get(parent));
        }

        println!("No action found for chord: {}", chord_str);
        None
    }

    fn resolve_semantic_action(&self, action: &contexts::SemanticAction, context: &contexts::Context) -> Vec<KeyEvent> {
        match action {
            contexts::SemanticAction::Sequence(actions) => {
                actions.iter().flat_map(|a| self.resolve_semantic_action(a, context)).collect()
            }
            contexts::SemanticAction::Action(action_name) => {
                if let Some(action) = context.semantic_actions.get(action_name) {
                    self.resolve_semantic_action(action, context)
                } else if let Some(parent) = &context.parent {
                    if let Some(parent_context) = self.contexts.get(parent) {
                        self.resolve_semantic_action(action, parent_context)
                    } else {
                        Vec::new()
                    }
                } else {
                    Vec::new()
                }
            }
            contexts::SemanticAction::KeyEvent(keys) => {
                vec![keys.clone()]
            }
            contexts::SemanticAction::LiteralString(s) => {
                //vec![s.clone()]
                //FIXME: Iterate over the string and return a sequence of key events.
                Vec::new()
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_dir = PathBuf::from("src/user_config");
    let mut engine = PinkyTwirlEngine::new(config_dir);

    engine.load_configurations()?;
    engine.print_config();

    // Example usage of get_context
    println!();
    println!();

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
    
    // Example usage of handle_key_event
    println!();
    println!();

    // let test_events = vec![
    //     key_down("meta"),
    //     key_down("meta + j"),
    //     key_up("meta + j"),
    //     key_up("meta")
    // ];

    // let test_events = vec![
    //     key_down("j"),
    //     key_up("j")
    // ];

    let test_events = vec![
        key_down("meta"),
        key_down("meta + tab")
    ];

    for event in test_events {
        println!("Event: {}", event);
        let synthetic_events = engine.handle_key_event(event.clone(), "Visual Studio Code", "main.rs - MyProject");
        println!("Synthetic events: {:?}", synthetic_events);
        println!();
    }

    Ok(())
}