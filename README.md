# Gravity UI Framework

[![Crates.io](https://img.shields.io/crates/v/gravity-core.svg)](https://crates.io/crates/gravity-core)
[![Documentation](https://docs.rs/gravity-core/badge.svg)](https://docs.rs/gravity-core)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Rust Version](https://img.shields.io/badge/rustc-1.75+-lightgray.svg)](https://rust-lang.org)

**Framework UI déclaratif pour Rust avec backend Iced, supportant le hot-reload, le styling avancé, et la génération de code.**

Gravity permet de définir votre interface utilisateur en XML et de l'afficher via Iced.

## Fonctionnalités

- ✅ **Definitions XML déclaratives**
- ✅ **Mode hot-reload** (<500ms de mise à jour)
- ✅ **Systeme de styling avancé** (themes, classes, styles etat)
- ✅ **Design responsive** avec breakpoints (mobile, tablet, desktop)
- ✅ **Gestionnaires d'evenements type-safe**
- ✅ **Evaluation d'expressions** dans les attributs XML
- ✅ **Support complet des widgets Iced** (text, buttons, inputs, layouts, etc.)

## Installation

### Prerequisites

- Rust 1.75 ou superieur (stable)
- Edition 2021 ou 2024

### Configuration du projet

```toml
[workspace]
members = [
    "crates/gravity-core",
    "crates/gravity-macros", 
    "crates/gravity-iced",
    "crates/gravity-runtime",
    "crates/gravity-cli",
]

[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
gravity-core = { path = "crates/gravity-core" }
gravity-macros = { path = "crates/gravity-macros" }
gravity-iced = { path = "crates/gravity-iced" }
iced = "0.14"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Structure du projet

```
my-app/
├── Cargo.toml
├── src/
│   ├── main.rs           # Point d'entree de l'application
│   └── ui/
│       ├── mod.rs        # Module UI avec AppState
│       └── main.gravity  # Definition XML de l'interface
└── target/
```

## Demarrage rapide

### 1. Creer le fichier UI (`src/ui/main.gravity`)

```xml
<?xml version="1.0" encoding="UTF-8" ?>
<gravity>
    <column padding="40" spacing="20">
        <text value="Hello, Gravity!" size="32" weight="bold" />
        <text value="Framework UI declaratif pour Rust" />
        <button label="Click me!" on_click="greet" />
    </column>
</gravity>
```

### 2. Creer le module UI (`src/ui/mod.rs`)

```rust
use gravity_macros::{gravity_ui, ui_handler, UiModel};
use serde::{Deserialize, Serialize};

#[derive(UiModel, Default, Serialize, Deserialize, Clone)]
struct Model {
    greeting: String,
}

#[gravity_ui("main.gravity")]
mod _app {}

#[ui_handler]
fn greet(model: &mut Model) {
    model.greeting = "Hello from Gravity!".to_string();
}

pub fn create_app_state() -> gravity_core::AppState<Model> {
    let document = _app::document();
    let registry = create_handler_registry();
    gravity_core::AppState::with_handlers(document, registry)
}

pub fn create_handler_registry() -> gravity_core::HandlerRegistry {
    use gravity_core::HandlerEntry;
    let registry = gravity_core::HandlerRegistry::new();
    
    registry.register_simple("greet", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            greet(m);
        }
    });
    
    registry
}
```

### 3. Creer le point d'entree (`src/main.rs`)

```rust
mod ui;

use gravity_core::AppState;
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use iced::{Element, Task};

type Message = HandlerMessage;

struct HelloApp {
    state: AppState<ui::Model>,
}

fn update(app: &mut HelloApp, message: Message) -> Task<Message> {
    match message {
        HandlerMessage::Handler(handler_name, _) => {
            if let Some(HandlerEntry::Simple(h)) = 
                app.state.handler_registry.get(&handler_name)
            {
                h(&mut app.state.model);
            }
        }
    }
    Task::none()
}

fn view(app: &HelloApp) -> Element<'_, Message> {
    GravityWidgetBuilder::new(
        &app.state.document,
        &app.state.model,
        Some(&app.state.handler_registry),
    )
    .build()
}

fn main() -> iced::Result {
    let state = ui::create_app_state();
    let app = HelloApp { state };
    iced::application(app, update, view).run()
}
```

### 4. Lancer l'application

```bash
cargo run
```

## Fonctionnalites avancees

### Liaison de donnees (Data Binding)

```rust
#[derive(UiModel, Default, Serialize, Deserialize, Clone)]
struct Model {
    count: i32,
    name: String,
    items: Vec<TodoItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
}
```

### Gestionnaires d'evenements types

```rust
#[ui_handler]
fn increment(model: &mut Model) {
    model.count += 1;
}

#[ui_handler]
fn add_item(model: &mut Model, text: String) {
    model.items.push(TodoItem {
        id: model.next_id,
        text,
        completed: false,
    });
    model.next_id += 1;
}

#[ui_handler]
fn toggle_item(model: &mut Model, id: usize) {
    if let Some(item) = model.items.iter_mut().find(|i| i.id == id) {
        item.completed = !item.completed;
    }
}
```

### Systeme de theming avance

```xml
<themes>
    <theme name="light">
        <palette 
            primary="#3498db" 
            secondary="#2ecc71"
            background="#ecf0f1"
            text="#2c3e50" />
        <typography font_family="Inter, sans-serif" font_size_base="16" />
        <spacing unit="8" />
    </theme>
    
    <theme name="dark">
        <palette 
            primary="#5dade2" 
            secondary="#52be80"
            background="#2c3e50"
            text="#ecf0f1" />
        <typography font_family="Inter, sans-serif" font_size_base="16" />
        <spacing unit="8" />
    </theme>
</themes>

<global_theme name="light" />
```

### Classes de style reutilisables

```xml
<styles>
    <style name="btn_primary">
        <base
            background="#3498db"
            color="#ffffff"
            padding="8 16"
            border_radius="6"
            border_width="0" />
        <hover background="#5dade2" />
        <active background="#2874a6" />
        <disabled opacity="0.5" />
    </style>
    
    <style name="btn_danger">
        <base
            background="#e74c3c"
            color="#ffffff"
            padding="8 16"
            border_radius="6" />
        <hover background="#ec7063" />
    </style>
</styles>

<button class="btn_primary" label="Valider" on_click="submit" />
<button class="btn_danger" label="Supprimer" on_click="delete" />
```

### Design responsive avec breakpoints

```xml
<column 
    mobile:spacing="10"
    tablet:spacing="15"
    desktop:spacing="20">
    <text 
        mobile:size="18"
        desktop:size="32"
        value="Texte responsive" />
</column>
```

### Widgets disponibles

| Widget | Description | Attributs principaux |
|--------|-------------|---------------------|
| `text` | Affichage de texte | value, size, weight, color |
| `button` | Bouton interactif | label, on_click, enabled, class |
| `text_input` | Champ de saisie | value, on_input, placeholder |
| `checkbox` | Case a cocher | checked, on_toggle |
| `toggler` | Interrupteur | active, on_toggle, label |
| `pick_list` | Liste deroulante | options, selected, on_select |
| `column` | Layout vertical | spacing, padding, align |
| `row` | Layout horizontal | spacing, padding, align |
| `scrollable` | Zone defilement | width, height |
| `container` | Conteneur | padding, width, height |
| `for` | Boucle dynamique | each, in |
| `grid` | Grille | columns, spacing |
| `progress_bar` | Barre de progression | min, max, value |
| `canvas` | Zone de dessin | width, height, program |
| `svg` | Image SVG | path, width, height |
| `tooltip` | Infobulle | message, position |

## Architecture

### Structure des crates

```
crates/
├── gravity-core/         # Parser XML, IR, traits (sans dependance Iced)
├── gravity-macros/       # Macros #[derive(UiModel)], #[ui_handler], #[gravity_ui]
├── gravity-runtime/      # Hot-reload, surveillance de fichiers
├── gravity-iced/         # Implementation backend Iced
└── gravity-cli/          # CLI developpeur (dev, build, check, inspect)

examples/
├── hello-world/          # Application minimale
├── counter/              # Gestionnaires interactifs
├── todo-app/             # Liaison de donnees complete
├── styling/              # Themes et classes de style
├── responsive/           # Design responsive
├── settings/             # Vues multiples
├── widget-showcase/      # Demonstration des widgets
├── builder-demo/         # Patterns widget custom
└── hot-reload-test/      # Workflow hot-reload

specs/
└── 001-006-*/            # Specifications techniques
```

### Principes fondamentaux

1. **Declaratif-First**: Le XML est la source de verite pour la structure UI
2. **Securite de type**: Pas d'erasure de type pour les messages/etat
3. **Dual-Mode**: Dev (hot-reload) + Prod (codegen)
4. **Backend-agnostic**: Le crate core n'a pas de dependance Iced
5. **Test-First**: TDD pour toutes les fonctionnalites

### Flux de donnees

```
XML (main.gravity)
        |
        v
    Parser XML
        |
        v
   GravityDocument
        |
        v
GravityWidgetBuilder
        |
        v
    Element<Iced>
```

## Exemples

Voir le repertoire [examples/](examples/) pour des demonstrations progressives :

| Exemple | Fonctionnalites |
|---------|-----------------|
| **hello-world** | Rendu UI statique minimal |
| **counter** | Gestionnaires d'evenements interactifs |
| **todo-app** | Liaison de donnees complete avec listes |
| **styling** | Themes, classes, styles etat |
| **responsive** | Design responsive avec breakpoints |
| **settings** | Vues multiples et navigation |
| **widget-showcase** | Demonstration de tous les widgets |

## Commandes CLI

```bash
# Mode developpement avec hot-reload
gravity dev --ui ui --file main.gravity --verbose

# Generer du code de production
gravity build --ui ui --output src/ui_genere.rs

# Valider les fichiers UI sans executer
gravity check --ui ui

# Inspecter l'IR ou le code genere
gravity inspect --file ui/main.gravity
gravity inspect --file ui/main.gravity --codegen --handlers increment,decrement
```

## Performance

| Metrique | Cible |
|----------|-------|
| Parse XML | <10ms pour 1000 widgets |
| Hot-reload | <500ms de la sauvegarde a la mise a jour UI |
| Generation de code | <5s pour une application typique |
| Memoire runtime (dev) | <50MB de base |

## Documentation

- **[Documentation API](https://docs.rs/gravity-core)** - Rustdoc complet
- **[Reference Schema XML](specs/001-framework-technical-specs/contracts/xml-schema.md)** - Widgets et attributs
- **[Guide de style](examples/styling/README.md)** - Themes, classes, styles etat
- **[Exemples](examples/)** - Projets examples progressifs

## Licence

Ce projet est sous licence Apache 2.0 ou MIT, au choix.

---

**Build avec ❤️ utilisant Rust et Iced**
