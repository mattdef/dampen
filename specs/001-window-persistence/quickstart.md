# Quickstart: Window State Persistence

**Feature**: 001-window-persistence
**Date**: 2026-01-24

This guide shows how to add window state persistence to a Dampen application.

## Option 1: Manual Integration (Recommended for learning)

Add persistence to an existing Dampen application with full control over the behavior.

### Step 1: Load saved state in main.rs

```rust
use dampen_dev::persistence::{load_or_default, save_window_state, WindowState};

fn main() -> iced::Result {
    // Load saved window state (or use defaults: 800x600)
    let window_state = load_or_default("my-app", 800, 600);

    let mut app = iced::application(MyApp::init, MyApp::update, MyApp::view)
        .title("My Dampen App")
        .window_size(window_state.size());

    // Apply position if available (skipped on Wayland)
    if let Some(pos) = window_state.position() {
        app = app.position(pos);
    } else {
        app = app.centered();
    }

    app.subscription(MyApp::subscription).run()
}
```

### Step 2: Track window geometry in your model

```rust
struct MyApp {
    // ... your existing fields ...

    // Window geometry tracking
    window_width: u32,
    window_height: u32,
    window_x: Option<i32>,
    window_y: Option<i32>,
    is_maximized: bool,
}

impl MyApp {
    fn init() -> (Self, Task<Message>) {
        let state = load_or_default("my-app", 800, 600);
        (
            Self {
                window_width: state.width,
                window_height: state.height,
                window_x: state.x,
                window_y: state.y,
                is_maximized: state.maximized,
                // ... other fields ...
            },
            Task::none(),
        )
    }
}
```

### Step 3: Handle window events

```rust
#[derive(Debug, Clone)]
enum Message {
    // ... your existing messages ...
    Window(iced::window::Event),
}

impl MyApp {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Window(event) => match event {
                iced::window::Event::Resized(size) => {
                    self.window_width = size.width as u32;
                    self.window_height = size.height as u32;
                    Task::none()
                }
                iced::window::Event::Moved(position) => {
                    self.window_x = Some(position.x as i32);
                    self.window_y = Some(position.y as i32);
                    Task::none()
                }
                iced::window::Event::CloseRequested => {
                    // Save state before closing
                    let state = WindowState {
                        width: self.window_width,
                        height: self.window_height,
                        x: self.window_x,
                        y: self.window_y,
                        maximized: self.is_maximized,
                    };
                    if let Err(e) = save_window_state("my-app", &state) {
                        tracing::warn!("Failed to save window state: {e}");
                    }
                    iced::window::close(iced::window::Id::MAIN)
                }
                _ => Task::none(),
            },
            // ... handle other messages ...
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::window::events().map(|(_, event)| Message::Window(event))
    }
}
```

---

## Option 2: Macro Integration (Simplest)

Enable persistence with a single attribute in the `#[dampen_app]` macro.

### Update your macro attributes

```rust
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    // ... existing attributes ...
    persistence = true,        // Enable automatic window persistence
    app_name = "my-app",       // Required: identifies your app's config file
)]
struct MyApp;
```

That's it. The macro generates all the window event handling and state tracking for you.

### What the macro generates

When `persistence = true`, the macro:
1. Adds a `Window(iced::window::Event)` subscription
2. Tracks window size/position via `Resized`/`Moved` events
3. Saves state to disk when `CloseRequested` is received
4. Loads saved state during `init()`

---

## Verifying It Works

### Check the config file location

```rust
use dampen_dev::persistence::get_config_path;

if let Some(path) = get_config_path("my-app") {
    println!("Config stored at: {}", path.display());
}
```

Expected locations:
- **Linux**: `~/.config/my-app/window.json`
- **Windows**: `C:\Users\{user}\AppData\Roaming\my-app\window.json`
- **macOS**: `~/Library/Application Support/my-app/window.json`

### Inspect the saved state

After closing your app, you can inspect the JSON file:

```bash
cat ~/.config/my-app/window.json
```

```json
{
  "width": 1024,
  "height": 768,
  "x": 200,
  "y": 100,
  "maximized": false
}
```

### Test the persistence

1. Run your app
2. Resize and move the window
3. Close the app
4. Run the app again
5. Verify the window appears at the saved size and position

---

## Troubleshooting

### Window always opens at default size

**Possible causes**:
- Config file doesn't exist yet (first run)
- Config file is corrupted or invalid
- Permission issue reading the config directory

**Debug**:
```rust
let state = load_or_default("my-app", 800, 600);
println!("Loaded state: {:?}", state);
println!("Config path: {:?}", get_config_path("my-app"));
```

### Window position is ignored (Linux)

If you're running on Wayland, window positioning is handled by the compositor. Size and maximized state will still work, but position is determined by Wayland.

### State not saved on close

Ensure you're handling `CloseRequested` and calling `save_window_state()` before `iced::window::close()`.

---

## Full Example

See `examples/hello-world/src/main.rs` for a complete working example with window persistence enabled.
