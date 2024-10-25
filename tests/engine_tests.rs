use pinkytwirl::{KeyEvent, KeyState, PinkyTwirlEngine};
use std::path::PathBuf;

// Helper functions
fn key_down(key: &str) -> KeyEvent {
    let (key_str, mut shift, mut ctrl, mut alt, mut meta) = parse_key_string(key);
    KeyEvent {
        key: key_str,
        state: KeyState::Down,
        shift,
        ctrl,
        alt,
        meta,
    }
}

fn key_up(key: &str) -> KeyEvent {
    let (key_str, mut shift, mut ctrl, mut alt, mut meta) = parse_key_string(key);
    KeyEvent {
        key: key_str,
        state: KeyState::Up,
        shift,
        ctrl,
        alt,
        meta,
    }
}

fn parse_key_string(key: &str) -> (String, bool, bool, bool, bool) {
    let parts: Vec<&str> = key.split(" + ").collect();
    let mut shift = false;
    let mut ctrl = false;
    let mut alt = false;
    let mut meta = false;

    let key_str = if parts.len() > 1 {
        for &part in &parts[..parts.len() - 1] {
            match part.to_lowercase().as_str() {
                "shift" => shift = true,
                "ctrl" => ctrl = true,
                "alt" => alt = true,
                "meta" => meta = true,
                _ => (),
            }
        }
        parts[parts.len() - 1].to_string()
    } else {
        parts[0].to_string()
    };

    (key_str, shift, ctrl, alt, meta)
}

#[test]
fn test_context_matching() {
    let config_dir = PathBuf::from("src/user_config");
    let mut engine = PinkyTwirlEngine::new(config_dir);
    engine
        .load_configurations()
        .expect("Failed to load configurations");

    let test_cases = vec![
        ("Visual Studio Code", "main.rs - MyProject", Some("VSCode")),
        ("FIREFOX", "Google - Mozilla Firefox", Some("Firefox")),
        ("cmd", "Command Prompt", Some("CommandPrompt")),
        (
            "notepad++",
            "config.txt - Notepad++",
            Some("NotepadPlusPlus"),
        ),
        ("unknown_app", "Unknown Window", None),
    ];

    for (app_name, window_name, expected_context) in test_cases {
        match engine.get_context(app_name, window_name) {
            Some(context) => {
                assert_eq!(
                    Some(context.name.as_str()),
                    expected_context,
                    "Unexpected context match for '{}' - '{}'",
                    app_name,
                    window_name
                );
            }
            None => {
                assert_eq!(
                    None, expected_context,
                    "Expected no context match for '{}' - '{}'",
                    app_name, window_name
                );
            }
        }
    }
}

#[test]
fn test_key_events() {
    let config_dir = PathBuf::from("src/user_config");
    let mut engine = PinkyTwirlEngine::new(config_dir);
    engine
        .load_configurations()
        .expect("Failed to load configurations");

    // Test meta key sequence
    let meta_sequence = vec![key_down("meta"), key_down("meta + tab")];

    for event in meta_sequence {
        let synthetic_events =
            engine.handle_key_event(event.clone(), "Visual Studio Code", "main.rs - MyProject");
        // Add assertions based on expected behavior
        match event.key.as_str() {
            "meta" => assert!(synthetic_events.is_empty(), "Meta key should be suppressed"),
            "tab" => assert!(
                !synthetic_events.is_empty(),
                "Meta + Tab should generate synthetic events"
            ),
            _ => panic!("Unexpected key in test sequence"),
        }
    }

    // Test simple key sequence
    let simple_sequence = vec![key_down("j"), key_up("j")];

    for event in simple_sequence {
        let synthetic_events =
            engine.handle_key_event(event.clone(), "Visual Studio Code", "main.rs - MyProject");
        assert!(
            synthetic_events.is_empty(),
            "Simple key events should pass through"
        );
    }
}

#[test]
fn test_chord_resolution() {
    let config_dir = PathBuf::from("src/user_config");
    let mut engine = PinkyTwirlEngine::new(config_dir);
    engine
        .load_configurations()
        .expect("Failed to load configurations");

    let chord_sequence = vec![
        key_down("meta"),
        key_down("meta + j"),
        key_up("meta + j"),
        key_up("meta"),
    ];

    let mut synthetic_events_found = false;
    for event in chord_sequence {
        let synthetic_events =
            engine.handle_key_event(event.clone(), "Visual Studio Code", "main.rs - MyProject");
        if !synthetic_events.is_empty() {
            synthetic_events_found = true;
            // Add specific assertions about the expected synthetic events
        }
    }

    assert!(
        synthetic_events_found,
        "Chord sequence should generate at least one synthetic event"
    );
}

// Optional: Test configuration loading
#[test]
fn test_config_loading() {
    let config_dir = PathBuf::from("src/user_config");
    let mut engine = PinkyTwirlEngine::new(config_dir);
    assert!(
        engine.load_configurations().is_ok(),
        "Configuration loading should succeed"
    );
}

// Optional: Print configuration in debug builds
#[test]
#[cfg(debug_assertions)]
fn debug_print_config() {
    let config_dir = PathBuf::from("src/user_config");
    let mut engine = PinkyTwirlEngine::new(config_dir);
    engine
        .load_configurations()
        .expect("Failed to load configurations");
    engine.print_config();
}
