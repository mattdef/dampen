# Dampen UI Framework

[![Crates.io](https://img.shields.io/crates/v/dampen-core.svg)](https://crates.io/crates/dampen-core)
[![Documentation](https://docs.rs/dampen-core/badge.svg)](https://docs.rs/dampen-core)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Rust Version](https://img.shields.io/badge/rustc-1.75+-lightgray.svg)](https://rust-lang.org)

**Framework UI déclaratif pour Rust avec backend Iced, styling avancé, et génération de code.**

Dampen permet de définir votre interface utilisateur en XML et de l'afficher via Iced.

## Fonctionnalités

- ✅ **Definitions XML déclaratives**
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
    "crates/dampen-core",
    "crates/dampen-macros", 
    "crates/dampen-iced",
    "crates/dampen-runtime",
    "crates/dampen-cli",
]

[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
dampen-core = { path = "crates/dampen-core" }
dampen-macros = { path = "crates/dampen-macros" }
dampen-iced = { path = "crates/dampen-iced" }
iced = "0.14"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Structure du projet

```
my-app/
├── Cargo.toml
├── src/
│   ├── main.rs             # Point d'entree de l'application
│   └── ui/
│       ├── mod.rs          # Module UI avec AppState
│       ├── window.rs       # Code de la fenêtre
│       └── window.dampen   # Definition XML de l'interface
└── target/
```

## Demarrage rapide

### Creer un nouveau projet

Utilisez la commande CLI pour scaffold un nouveau projet Dampen :

```bash
# Creer un nouveau projet
dampen new my-app

# Naviguer vers le projet
cd my-app

# Lancer l'application
cargo run
```

La commande `dampen new` cree une structure de projet complete :

```
my-app/
├── Cargo.toml              # Dependencies du projet
├── README.md               # Guide de demarrage
├── build.rs                # Generation de code (XML → Rust)
├── src/
│   ├── main.rs             # Point d'entree de l'application
│   └── ui/
│       ├── mod.rs          # Exports du module UI
│       ├── window.rs       # Model et handlers avec #[dampen_ui]
│       └── window.dampen   # Definition UI declarative (XML)
└── tests/
    └── integration.rs      # Tests d'integration
```

**Fichiers cles :**

| Fichier | Description |
|---------|-------------|
| `src/ui/window.dampen` | Definition UI XML avec widgets, bindings, handlers |
| `src/ui/window.rs` | Model avec `#[derive(UiModel)]`, registre handlers |
| `src/main.rs` | Orchestration application (view, update) |
| `build.rs` | Compile les fichiers `.dampen` en code Rust |

**Exemple UI genere :**

```xml
<dampen>
    <column padding="40" spacing="20">
        <text value="Hello, Dampen!" size="32" weight="bold" />
        <button label="Click me!" on_click="greet" />
        <text value="{message}" size="24" />
    </column>
</dampen>
```

### Validation du projet

```bash
# Valider la syntaxe XML et les noms de widgets
dampen check

# Construire le projet
dampen build

# Inspecter l'IR genere
dampen inspect src/ui/window.dampen
```

### Structure du projet (manuelle)

Si vous preferez creer le projet manuellement :

```
my-app/
├── Cargo.toml
├── src/
│   ├── main.rs             # Point d'entree de l'application
│   └── ui/
│       ├── mod.rs          # Module UI avec AppState
│       ├── window.rs       # Code de la fenetre
│       └── window.dampen   # Definition XML de l'interface
└── target/
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
fn increment(model: &mut Model) {
    model.count += 1;
}

fn add_item(model: &mut Model, text: String) {
    model.items.push(TodoItem {
        id: model.next_id,
        text,
        completed: false,
    });
    model.next_id += 1;
}

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
| `svg` | Image SVG | path, width, height |
| `tooltip` | Infobulle | message, position |

## Architecture

### Structure des crates

```
crates/
├── dampen-core/           # Parser XML, IR, traits (sans dependance Iced)
├── dampen-macros/         # Macros #[derive(UiModel)], #[dampen_ui]
├── dampen-runtime/        # Interpretation, gestion d'etat, erreurs
├── dampen-iced/           # Implementation backend Iced
└── dampen-cli/            # CLI developpeur (build, check, inspect)

examples/
├── hello-world/           # Application minimale
├── counter/               # Gestionnaires d'evenements interactifs
├── todo-app/              # Liaison de donnees complete
├── styling/               # Themes et classes de style
├── responsive/            # Design responsive
├── settings/              # Vues multiples
├── widget-showcase/       # Demonstration des widgets
└── builder-demo/          # Patterns widget custom

specs/
└── 001-006-*/             # Specifications techniques
```

### Principes fondamentaux

1. **Declaratif-First**: Le XML est la source de verite pour la structure UI
2. **Securite de type**: Pas d'erasure de type pour les messages/etat
3. **Production Mode**: Code generation statique pour les deploiements
4. **Backend-agnostic**: Le crate core n'a pas de dependance Iced
5. **Test-First**: TDD pour toutes les fonctionnalites

### Flux de donnees

```
XML (main.dampen)
        |
        v
    Parser XML
        |
        v
   DampenDocument
        |
        v
  DampenWidgetBuilder
        |
        v
     Element<Iced>
```

## Exemples

Voir le repertoire [examples/](examples/) pour des demonstrations progressives :

| Exemple | Fonctionnalites |
|---------|----------------|
| **hello-world** | Rendu UI statique minimal |
| **counter** | Gestionnaires d'evenements interactifs |
| **todo-app** | Liaison de donnees complete avec listes |
| **styling** | Themes, classes, styles etat |
| **responsive** | Design responsive avec breakpoints |
| **settings** | Vues multiples et navigation |
| **widget-showcase** | Demonstration de tous les widgets |

## Commandes CLI

```bash
# Generer du code de production
dampen build --ui ui --output src/ui_genere.rs

# Valider les fichiers UI sans executer
dampen check --ui ui

# Inspecter l'IR ou le code genere
dampen inspect --file ui/main.dampen
dampen inspect --file ui/main.dampen --codegen --handlers increment,decrement
```

## Performance

| Metrique | Cible |
|----------|-------|
| Parse XML | <10ms pour 1000 widgets |
| Generation de code | <5s pour une application typique |
| Memoire runtime | <50MB de base |

## Documentation

- **[Documentation API](https://docs.rs/dampen-core)** - Rustdoc complet
- **[Reference Schema XML](specs/001-framework-technical-specs/contracts/xml-schema.md)** - Widgets et attributs
- **[Guide de style](examples/styling/README.md)** - Themes, classes, styles etat
- **[Exemples](examples/)** - Projets examples progressifs

## Licence

Ce projet est sous licence Apache 2.0 ou MIT, au choix.

---

**Build avec ❤️ utilisant Rust et Iced**
