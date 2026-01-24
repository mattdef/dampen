use dampen_dev::persistence::{WindowState, get_config_path, load_or_default, save_window_state};
use std::fs;
use uuid::Uuid;

fn random_app_name() -> String {
    format!("dampen-test-{}", Uuid::new_v4())
}

#[test]
fn test_load_or_default_missing_file() {
    let app_name = random_app_name();
    let state = load_or_default(&app_name, 800, 600);

    assert_eq!(state.width, 800);
    assert_eq!(state.height, 600);
    assert_eq!(state.maximized, false);
}

#[test]
fn test_load_or_default_valid_file() {
    let app_name = random_app_name();
    // This test relies on get_config_path returning a valid path
    // If it returns None (e.g. some CI envs), we skip
    if let Some(path) = get_config_path(&app_name) {
        // Create dir
        fs::create_dir_all(path.parent().unwrap()).unwrap();

        // Write JSON
        let json = r#"
        {
            "width": 1024,
            "height": 768,
            "x": 100,
            "y": 200,
            "maximized": true
        }
        "#;
        fs::write(&path, json).unwrap();

        let state = load_or_default(&app_name, 800, 600);

        assert_eq!(state.width, 1024);
        assert_eq!(state.height, 768);
        assert_eq!(state.x, Some(100));
        assert_eq!(state.y, Some(200));
        assert_eq!(state.maximized, true);

        // Cleanup
        let _ = fs::remove_file(&path);
        let _ = fs::remove_dir(path.parent().unwrap());
    }
}

#[test]
fn test_load_or_default_corrupted_file() {
    let app_name = random_app_name();
    if let Some(path) = get_config_path(&app_name) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "not json").unwrap();

        let state = load_or_default(&app_name, 800, 600);

        // Should fallback to defaults
        assert_eq!(state.width, 800);
        assert_eq!(state.height, 600);

        // Cleanup
        let _ = fs::remove_file(&path);
        let _ = fs::remove_dir(path.parent().unwrap());
    }
}

#[test]
fn test_save_window_state_creates_directories() {
    let app_name = random_app_name();
    if let Some(path) = get_config_path(&app_name) {
        // Ensure it doesn't exist (clean slate)
        if path.parent().unwrap().exists() {
            // Best effort cleanup
            let _ = fs::remove_dir_all(path.parent().unwrap());
        }

        let state = WindowState::with_defaults(800, 600);
        save_window_state(&app_name, &state).unwrap();

        assert!(path.exists());

        // Cleanup
        let _ = fs::remove_file(&path);
        let _ = fs::remove_dir(path.parent().unwrap());
    }
}

#[test]
fn test_save_window_state_writes_valid_json() {
    let app_name = random_app_name();
    if let Some(path) = get_config_path(&app_name) {
        let mut state = WindowState::with_defaults(800, 600);
        state.x = Some(50);

        save_window_state(&app_name, &state).unwrap();

        assert!(path.exists());
        let content = fs::read_to_string(&path).unwrap();
        let loaded: WindowState = serde_json::from_str(&content).unwrap();

        assert_eq!(loaded.width, 800);
        assert_eq!(loaded.x, Some(50));

        // Cleanup
        let _ = fs::remove_file(&path);
        let _ = fs::remove_dir(path.parent().unwrap());
    }
}

#[test]
fn test_window_state_wayland_scenario() {
    let state = WindowState {
        width: 800,
        height: 600,
        x: None,
        y: None,
        maximized: false,
    };

    assert!(state.position().is_none());

    let json = serde_json::to_string(&state).unwrap();
    assert!(!json.contains("\"x\":"));
    assert!(!json.contains("\"y\":"));
}

#[test]
fn test_window_state_maximized_deserialization() {
    let json = r#"{"width": 800, "height": 600, "maximized": true}"#;
    let state: WindowState = serde_json::from_str(json).unwrap();
    assert!(state.maximized);
}

#[test]
fn test_window_state_conversions() {
    let mut state = WindowState::with_defaults(800, 600);
    state.x = Some(100);
    state.y = Some(200);

    let size = state.size();
    assert_eq!(size.width, 800.0);
    assert_eq!(size.height, 600.0);

    let pos = state.position().unwrap();
    assert_eq!(pos.x, 100.0);
    assert_eq!(pos.y, 200.0);

    state.x = None;
    assert!(state.position().is_none());
}

#[test]
fn test_load_or_default_ignores_unreasonable_position() {
    let app_name = random_app_name();
    if let Some(path) = get_config_path(&app_name) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        // Extremely large position
        let json = r#"{"width": 800, "height": 600, "x": 100000, "y": 100000, "maximized": false}"#;
        fs::write(&path, json).unwrap();

        let state = load_or_default(&app_name, 800, 600);

        // Should ignore position (None) but keep size
        assert_eq!(state.width, 800);
        assert!(state.x.is_none(), "Should ignore x=100000");
        assert!(state.y.is_none(), "Should ignore y=100000");

        // Cleanup
        let _ = fs::remove_file(&path);
        let _ = fs::remove_dir(path.parent().unwrap());
    }
}
