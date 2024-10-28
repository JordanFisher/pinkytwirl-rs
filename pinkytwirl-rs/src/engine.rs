// [x] Load the JSON config files (YAML?).
// [x] Parse the inner semantics.
// [x] Context lookup.
// [x] KeyCombination(Vec<String>) should become Key(Key), which is just a key with boolean meta
// [x] include the key code?
// [x] State manager, input/output.
// [x] copy from claude, but return a Vec<Key> instead of Vec<String>
// [x] Basic tests with synthetic input/outputs.
// [ ] Get rest of FFI built and initial cycling
//     [x] Keep pressed keys up to date correctly.
//     [x] Ambiguous based on stems.
//     [x] Flag for once no mapping should be applied until full reset. Force reset on meta up.
//     [x] Flag for once only mappings should be applied until full reset. Force reset on meta up.
//     [x] Returns flag for whether to pass through the event or not.
//     [x] Key down from meta.
//     [x] Generate actual key events on macOS.
//     [x] Get 3/4 working
//     [x] Get prev/next word working
//     [x] Get undo/redo working
//     [ ] Reset shift on stem change?
//     [ ] Get meta+tab working (queue up actual key events?)
//     [ ] Get meta+space+j working
// [ ] Add README and MIT license
// [x] Embed into macOS
// [ ] chrome tab + desktop window switching, most recent, etc
// [ ] text suggestion, find location of cursor, etc
// [ ] Embed into windows (c wrapper? no c#?)
// [ ] Refactor to not need strings.

use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::path::PathBuf;

use crate::contexts::{Context, KeyEvent, KeyState, SemanticAction};

pub struct PinkyTwirlEngine {
    contexts: HashMap<String, Context>,
    config_dir: String,
    pub pressed_keys: VecDeque<KeyEvent>,
    current_context: Option<String>,
    keycodes: crate::keycode_macos::KeyCodeLookup,

    no_mapping_until_reset: bool,
    only_mappings_until_reset: bool,
    synthetic_keys: Vec<KeyEvent>,

    pub startup: Result<(), Box<dyn Error>>,
    debug_key_events: bool,
}

impl PinkyTwirlEngine {
    pub fn new(config_dir: String) -> Self {
        let mut engine = PinkyTwirlEngine {
            contexts: HashMap::new(),
            config_dir,
            pressed_keys: VecDeque::new(),
            current_context: None,
            keycodes: crate::keycode_macos::create_keycode_map(),

            no_mapping_until_reset: false,
            only_mappings_until_reset: false,
            synthetic_keys: Vec::new(),

            startup: Ok(()),
            debug_key_events: true,
        };

        engine.startup = engine.load_configurations();

        if let Err(e) = &engine.startup {
            eprintln!("Engine error loading configurations: {}", e);
        }
        engine
    }

    fn reset(&mut self) {
        self.pressed_keys.clear();
        self.current_context = None;
        self.no_mapping_until_reset = false;
        self.only_mappings_until_reset = false;
    }

    pub fn load_configurations(&mut self) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(self.config_dir.clone());

        // Get the path of the current executable.
        let exe_path = std::env::current_exe()?;
        println!("Current executable path: {:?}", exe_path);
        // Set the current working directory to the directory of the executable.
        let exe_dir = exe_path.parent().unwrap();
        std::env::set_current_dir(exe_dir)?;

        // Print the current working directory.
        let cwd = std::env::current_dir()?;
        println!("Current working directory: {:?}", cwd);
        println!("Loading configurations from: {:?}", path);

        // Fully resolve the path.
        let path = std::fs::canonicalize(path)?;
        println!("Resolved path for configurations: {:?}", path);

        let contexts_path = path.join("contexts.txt");
        println!("Loading contexts from: {:?}", contexts_path);
        self.contexts = crate::contexts::parse_yaml_file(&contexts_path)?;

        let semantics_path = path.join("semantics_macos.txt");
        println!("Loading semantics from: {:?}", semantics_path);
        crate::semantics::parse_semantics_file(
            &semantics_path,
            &mut self.contexts,
            &self.keycodes,
        )?;

        let mappings_path = path.join("mappings.txt");
        println!("Loading mappings from: {:?}", mappings_path);
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
            // Then try exact match with window name.
            .or_else(|| exact_match(window_name))
            // Then try case-insensitive match with app name.
            .or_else(|| case_insensitive_match(app_name))
            // Then try case-insensitive match with window name.
            .or_else(|| case_insensitive_match(window_name))
            // Then try substring match with app name.
            .or_else(|| substring_match(app_name))
            // Finally, try substring match with window name.
            .or_else(|| substring_match(window_name))
            // Otherwise we should look for a "Default" or "default" context.
            .or_else(|| self.contexts.get("Default"))
            .or_else(|| self.contexts.get("default"))
            .or_else(|| substring_match("default"))
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
    ) -> (bool, Vec<KeyEvent>) {
        if self.debug_key_events {
            println!(
                "Engine key event: {} {:?} {} {} {}",
                event.key, event.state, event.shift, event.ctrl, event.alt
            );
        }

        match event.state {
            KeyState::Down => {
                // Add the key to the pressed keys if it's not already there.
                if !self
                    .pressed_keys
                    .iter()
                    .any(|k| &k.key == &event.key && k.state == KeyState::Down)
                {
                    self.pressed_keys.push_back(event.clone());
                }

                if self.no_mapping_until_reset {
                    // If we're in a state where no mappings should be applied, always let the event through.
                    return (false, Vec::new());
                }

                let action = if let Some(context) = self.get_context(app_name, window_name) {
                    if self.debug_key_events {
                        println!("Engine context: {}", context.name);
                        println!("Engine pressed keys: {:?}", self.pressed_keys);
                    }

                    if let Some(action) = self.find_chord_action(&context, &self.pressed_keys) {
                        Some(action)
                    } else {
                        None
                    }
                } else {
                    if self.debug_key_events {
                        println!("No context found for app: {} window: {}", app_name, window_name);
                    }

                    // If no context is found, let the key through
                    return (false, Vec::new());
                };

                if let Some(action) = action {
                    if action != SemanticAction::Action("MappingStem".to_string()) {
                        // We have a real action, we don't want to consider this key part of the stem.
                        self.pressed_keys.retain(|k| &k.key != &event.key);
                    }
                    // There is a mapping for the current chord, so we will suppress the
                    // key events and play back the synthetic events instead. We will stay
                    // in this mode until all keys are released.
                    let context = self.get_context(app_name, window_name).unwrap();
                    let synthetic_events = self.resolve_semantic_action(&action, context);
                    self.only_mappings_until_reset = true;
                    return (true, synthetic_events);
                } else if self.pressed_keys.len() == 1 {
                    // If it's the first key and doesn't match any chord, let it through.
                    self.no_mapping_until_reset = true;
                    return (false, Vec::new());
                } else {
                    // If it doesn't match any chord, play back the buffered keys, unless we're in a state where only mappings should be applied.
                    if self.only_mappings_until_reset {
                        return (true, Vec::new());
                    } else {
                        let synthetic_events = self.pressed_keys.iter().cloned().collect();
                        self.no_mapping_until_reset = true;
                        return (false, synthetic_events);
                    }
                }
            }
            KeyState::Up => {
                self.pressed_keys.retain(|k| &k.key != &event.key);
                if event.key == "meta" || self.pressed_keys.is_empty() {
                    self.reset();
                }
                return (false, Vec::new());
            }
            KeyState::DownUp => {
                return (false, Vec::new());
            }
        }
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
        key_code: i64,
        down: bool,
        shift: bool,
        ctrl: bool,
        option: bool,
        meta: bool,
        app_name: &str,
        window_name: &str,
    ) -> bool {
        let key_name = self
            .keycodes
            .keycode_to_name
            .get(&key_code)
            .unwrap_or(&"Unknown".to_string())
            .clone();
        let event = KeyEvent {
            key: key_name,
            code: key_code,
            state: if down { KeyState::Down } else { KeyState::Up },
            shift,
            ctrl,
            alt: option,
            meta,
        };
        let (suppress, synthetic_keys) = self.handle_key_event(event, app_name, window_name);
        
        // Convert DownUp events to a Down event followed by an Up event.
        self.synthetic_keys = synthetic_keys
            .iter()
            .flat_map(|key| match key.state {
                KeyState::DownUp => vec![
                    KeyEvent {
                        key: key.key.clone(),
                        code: key.code,
                        state: KeyState::Down,
                        shift: key.shift,
                        ctrl: key.ctrl,
                        alt: key.alt,
                        meta: key.meta,
                    },
                    KeyEvent {
                        key: key.key.clone(),
                        code: key.code,
                        state: KeyState::Up,
                        shift: key.shift,
                        ctrl: key.ctrl,
                        alt: key.alt,
                        meta: key.meta,
                    },
                ],
                _ => vec![key.clone()],
            })
            .collect();
        
        // Add the macOS key code to the synthetic keys.
        for key in &mut self.synthetic_keys {
            key.code = self
                .keycodes
                .name_to_keycode
                .get(&key.key)
                .cloned()
                .unwrap_or(0);
        }

        // Return whether the event should be suppressed.
        suppress
    }

    pub fn get_synthetic_events(&mut self) -> Vec<KeyEvent> {
        let synthetic_keys = self.synthetic_keys.clone();
        self.synthetic_keys.clear();
        synthetic_keys
    }
}
