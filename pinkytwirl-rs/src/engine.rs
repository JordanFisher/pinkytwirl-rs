// [x] Load the JSON config files (YAML?).
// [x] Parse the inner semantics.
// [x] Context lookup.
// [x] KeyCombination(Vec<String>) should become Key(Key), which is just a key with boolean meta
// [x] include the key code?
// [x] State manager, input/output.
// [x] copy from claude, but return a Vec<Key> instead of Vec<String>
// [x] Basic tests with synthetic input/outputs.
// [ ] Get rest of FFI built and initial cycling
// [ ] Returns flag for whether to pass through the event or not?
// [ ] Keep chord active while pressed.
// [x] Embed into macOS
// [ ] chrome tab + desktop window switching, most recent, etc
// [ ] text suggestion, find location of cursor, etc
// [ ] Embed into windows

use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::path::PathBuf;

use crate::contexts::{Context, KeyEvent, KeyState, SemanticAction};

pub struct PinkyTwirlEngine {
    contexts: HashMap<String, Context>,
    config_dir: String,
    pressed_keys: VecDeque<KeyEvent>,
    current_context: Option<String>,
    keycodes: crate::keycode_macos::KeyCodeLookup,
}

impl PinkyTwirlEngine {
    pub fn new(config_dir: String) -> Self {
        PinkyTwirlEngine {
            contexts: HashMap::new(),
            config_dir,
            pressed_keys: VecDeque::new(),
            current_context: None,
            keycodes: crate::keycode_macos::create_keycode_map(),
        }
    }

    pub fn load_configurations(&mut self) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(self.config_dir.clone());
        let contexts_path = path.join("contexts.txt");
        self.contexts = crate::contexts::parse_yaml_file(&contexts_path)?;

        let semantics_path = path.join("semantics.txt");
        crate::semantics::parse_semantics_file(
            &semantics_path,
            &mut self.contexts,
            &self.keycodes,
        )?;

        let mappings_path = path.join("mappings.txt");
        crate::mappings::parse_mappings_file(&mappings_path, &mut self.contexts, &self.keycodes)?;

        Ok(())
    }

    pub fn get_context(&self, app_name: &str, window_name: &str) -> Option<&Context> {
        // Helper function for exact match
        let exact_match = |name: &str| -> Option<&Context> {
            self.contexts
                .values()
                .find(|c| c.aliases.contains(&name.to_string()))
        };

        // Helper function for case-insensitive match
        let case_insensitive_match = |name: &str| -> Option<&Context> {
            let lower_name = name.to_lowercase();
            self.contexts.values().find(|c| {
                c.aliases
                    .iter()
                    .any(|alias| alias.to_lowercase() == lower_name)
            })
        };

        // Helper function for substring match
        let substring_match = |name: &str| -> Option<&Context> {
            let lower_name = name.to_lowercase();
            self.contexts.values().find(|c| {
                c.aliases.iter().any(|alias| {
                    let lower_alias = alias.to_lowercase();
                    lower_alias.contains(&lower_name) || lower_name.contains(&lower_alias)
                })
            })
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

    pub fn handle_key_event(
        &mut self,
        event: KeyEvent,
        app_name: &str,
        window_name: &str,
    ) -> Vec<KeyEvent> {
        let mut synthetic_events = Vec::new();

        match event.state {
            KeyState::Down => {
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
            KeyState::Up => {
                self.pressed_keys.retain(|k| &k.key != &event.key);
                if self.pressed_keys.is_empty() {
                    self.current_context = None;
                }
            }
            KeyState::DownUp => {}
        }

        synthetic_events
    }

    fn find_chord_action<'a>(
        &self,
        context: &'a Context,
        chord: &VecDeque<KeyEvent>,
    ) -> Option<SemanticAction> {
        let chord_str = chord
            .iter()
            .map(|key| key.key.clone())
            .collect::<Vec<String>>()
            .join(" + ");
        dbg!(&chord_str);
        // Lookup the chord in the context's key mappings. If it's not found, try the parent context, then the parent parent, etc.
        let mut current_context = Some(context);
        while let Some(context) = current_context {
            if let Some(action) = context.key_mappings.get(&chord_str) {
                println!("Found action: {}", action);
                return Some(action.clone());
            }
            current_context = context
                .parent
                .as_ref()
                .and_then(|parent| self.contexts.get(parent));
        }

        println!("No action found for chord: {}", chord_str);
        None
    }

    fn resolve_semantic_action(&self, action: &SemanticAction, context: &Context) -> Vec<KeyEvent> {
        dbg!(&action);
        match action {
            SemanticAction::Sequence(actions) => actions
                .iter()
                .flat_map(|a| self.resolve_semantic_action(a, context))
                .collect(),
            SemanticAction::Action(action_name) => {
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
            SemanticAction::KeyEvent(keys) => {
                vec![keys.clone()]
            }
            SemanticAction::LiteralString(s) => {
                //vec![s.clone()]
                //FIXME: Iterate over the string and return a sequence of key events.
                Vec::new()
            }
        }
    }

    pub fn macos_handle_key_event(
        &mut self,
        key_code: u16,
        down: bool,
        shift: bool,
        ctrl: bool,
        option: bool,
        meta: bool,
        app_name: &str,
        window_name: &str,
    ) -> Vec<KeyEvent> {
        let key_name = self
            .keycodes
            .keycode_to_name
            .get(&key_code)
            .unwrap_or(&"Unknown".to_string())
            .clone();
        let event = KeyEvent {
            key: key_name,
            state: if down { KeyState::Down } else { KeyState::Up },
            shift,
            ctrl,
            alt: option,
            meta,
        };
        self.handle_key_event(event, app_name, window_name)
    }
}
