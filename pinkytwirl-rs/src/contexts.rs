use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;

use crate::keycode_macos::KeyCodeLookup;

#[derive(Debug, Deserialize, Serialize)]
pub struct YamlContext {
    pub aliases: Vec<String>,
    pub parent: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyState {
    Up,
    Down,
    DownUp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: String,
    pub code: i64,
    pub state: KeyState,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
    pub func: bool,
    pub modifier_down_only: bool,
}

impl KeyEvent {
    pub fn get_code(&self) -> i64 {
        self.code
    }

    pub fn is_down(&self) -> bool {
        self.state == KeyState::Down || self.state == KeyState::DownUp
    }

    pub fn get_shift(&self) -> bool {
        self.shift
    }

    pub fn get_ctrl(&self) -> bool {
        self.ctrl
    }

    pub fn get_alt(&self) -> bool {
        self.alt
    }

    pub fn get_meta(&self) -> bool {
        self.meta
    }

    pub fn get_fn(&self) -> bool {
        self.func
    }
}

impl fmt::Display for KeyEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut modifiers = Vec::new();
        if self.ctrl {
            modifiers.push("Ctrl");
        }
        if self.shift {
            modifiers.push("Shift");
        }
        if self.alt {
            modifiers.push("Alt");
        }
        if self.meta {
            modifiers.push("Meta");
        }

        let key_str = if modifiers.is_empty() {
            self.key.clone()
        } else {
            format!("{} + {}", modifiers.join(" + "), self.key)
        };

        write!(f, "KeyEvent({}, {:?})", key_str, self.state)?;

        Ok(())
    }
}

pub fn key_press(s: &str) -> KeyEvent {
    let parts: Vec<String> = s
        .split('+')
        .map(|s| s.trim().to_lowercase().to_string())
        .collect();

    let mut key = "";
    let mut shift = false;
    let mut ctrl = false;
    let mut alt = false;
    let mut meta = false;
    let mut func = false;

    let mut modifier_down_only = false;
    for part in parts.iter() {
        match part.as_str() {
            "shift" => shift = true,
            "ctrl" => ctrl = true,
            "alt" => alt = true,
            "meta" => meta = true,
            "metadown" => { meta = true; modifier_down_only = true; },
            "fn" => func = true,
            _ => key = part,
        }
    }

    if parts.len() == 1 {
        key = parts[0].as_str();
    }

    KeyEvent {
        key: key.to_string(),
        code: 0,
        state: KeyState::DownUp,
        shift: shift,
        ctrl: ctrl,
        alt: alt,
        meta: meta,
        func: func,
        modifier_down_only: modifier_down_only,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticAction {
    Sequence(Vec<SemanticAction>),
    Action(String),
    KeyEvent(KeyEvent),
    LiteralString(String),
}

impl fmt::Display for SemanticAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemanticAction::Sequence(actions) => {
                write!(f, "Sequence(")?;
                for (i, action) in actions.iter().enumerate() {
                    if i < actions.len() - 1 {
                        write!(f, "{} | ", action)?;
                    } else {
                        write!(f, "{}", action)?;
                    }
                }
                write!(f, ")")
            }
            SemanticAction::Action(action) => write!(f, "Action({})", action),
            SemanticAction::KeyEvent(event) => write!(f, "{}", event),
            SemanticAction::LiteralString(s) => write!(f, "LiteralString(\"{}\")", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub name: String,
    pub aliases: Vec<String>,
    pub parent: Option<String>,
    pub semantic_actions: HashMap<String, SemanticAction>,
    pub key_mappings: HashMap<String, SemanticAction>,
}

pub fn parse_yaml_file(
    file_path: &Path,
) -> Result<HashMap<String, Context>, Box<dyn std::error::Error>> {
    let yaml_str = fs::read_to_string(file_path)?;
    Ok(parse_yaml(&yaml_str)?)
}

pub fn parse_yaml(yaml_str: &str) -> Result<HashMap<String, Context>, serde_yaml::Error> {
    let debug = false;
    let warn = true;

    let yaml_contexts: HashMap<String, YamlContext> = serde_yaml::from_str(yaml_str)?;
    let mut contexts = HashMap::new();

    // First pass: Create Context objects.
    for (name, yaml_context) in &yaml_contexts {
        if debug {
            println!("Creating context: {}", name);
        }

        contexts.insert(
            name.clone(),
            Context {
                name: name.clone(),
                aliases: yaml_context.aliases.clone(),
                parent: yaml_context.parent.clone(),
                semantic_actions: HashMap::new(),
                key_mappings: HashMap::new(),
            },
        );
    }

    // Second pass: Make sure that parent contexts exists if set.
    let context_names: Vec<String> = contexts.keys().cloned().collect();
    for context in contexts.values_mut() {
        if let Some(parent_name) = &context.parent {
            if !context_names.contains(parent_name) {
                if warn {
                    println!(
                        "Warning: Parent context '{}' not found for context '{}'. Skipping.",
                        parent_name, context.name
                    );
                }

                context.parent = None;
            }
        }
    }

    Ok(contexts)
}

pub fn parse_semantic_action(input: &str, keycodes: &KeyCodeLookup) -> SemanticAction {
    let parts: Vec<&str> = input.split('|').map(str::trim).collect();
    let mut sequence = Vec::new();

    for part in parts {
        let part = part.trim().to_lowercase().to_string();
        if part.starts_with('"') && part.ends_with('"') {
            sequence.push(SemanticAction::LiteralString(
                part[1..part.len() - 1].to_string(),
            ));
        } else if part.contains('+') {
            sequence.push(SemanticAction::KeyEvent(key_press(&part)));
        } else if part.contains('*') {
            let (count, key) = part.split_once('*').unwrap();
            let count: usize = count.trim().parse().unwrap_or(1);
            let key = key.trim().to_string();
            for _ in 0..count {
                sequence.push(SemanticAction::KeyEvent(key_press(&key)));
            }
        } else if keycodes.name_to_keycode.contains_key(&part) {
            sequence.push(SemanticAction::KeyEvent(key_press(&part)));
        } else {
            sequence.push(SemanticAction::Action(part.to_string()));
        }
    }

    if sequence.len() == 1 {
        sequence.pop().unwrap()
    } else {
        SemanticAction::Sequence(sequence)
    }
}
