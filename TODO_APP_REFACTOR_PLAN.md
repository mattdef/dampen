# 📋 PLAN COMPLET: Refactor Todo-App vers Style Iced

## 🎯 Objectif

Refondre l'application `todo-app` pour qu'elle corresponde visuellement et fonctionnellement à l'exemple Iced "todos", tout en conservant les avantages de Dampen:
- ✅ Hot-reload (interpreted mode)
- ✅ Codegen (production mode)
- ✅ Shared-state (simplifié pour usage futur)

## 📂 Structure des Fichiers

### Fichiers à SUPPRIMER
```
examples/todo-app/src/ui/add_task.dampen
examples/todo-app/src/ui/add_task.rs
examples/todo-app/src/ui/statistics.dampen
examples/todo-app/src/ui/statistics.rs
```

### Fichiers à MODIFIER

| Fichier | Type de modification |
|----------|---------------------|
| `examples/todo-app/src/main.rs` | Simplifier (remove CurrentView, single view) |
| `examples/todo-app/src/shared.rs` | Simplifier (remove complex fields) |
| `examples/todo-app/src/ui/window.rs` | Refactor complet du modèle et handlers |
| `examples/todo-app/src/ui/window.dampen` | Refactor complet vers design Iced |
| `examples/todo-app/Cargo.toml` | Ajuster si nécessaire |

---

## 🔧 Étape 1: Simplifier l'Architecture

### 1.1 Supprimer la multi-vue dans `main.rs`

**Actions:**
- Supprimer `CurrentView` enum (si elle existe dans main.rs)
- Simplifier `Message` enum pour n'avoir que:
  - `Handler(HandlerMessage)` - pour les handlers UI
  - `#[cfg(debug_assertions)] HotReload(FileEvent)` - pour dev
  - `#[cfg(debug_assertions)] DismissError` - pour les erreurs
- Mettre à jour `#[dampen_app]` macro pour single view:
  - `default_view = "window"`
  - Supprimer `switch_view_variant`
  - Supprimer `shared_model` si pas nécessaire

**Résultat attendu:**
```rust
#[derive(Clone, Debug)]
enum Message {
    Handler(HandlerMessage),
    #[cfg(debug_assertions)]
    HotReload(FileEvent),
    #[cfg(debug_assertions)]
    DismissError,
}

#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    hot_reload_variant = "HotReload",
    dismiss_error_variant = "DismissError",
    default_view = "window"
)]
struct TodosApp;
```

### 1.2 Ajouter subscription pour raccourcis clavier

**Actions:**
- Ajouter `Subscription<Message>` return type à `main()`
- Implémenter `subscription()` method dans `TodosApp` (ou macro-generated)
- Écouter les événements clavier pour:
  - `Tab` → focus suivant
  - `Shift+Tab` → focus précédent

**Résultat attendu:**
```rust
iced::application(TodosApp::init, TodosApp::update, TodosApp::view)
    .window_size(iced::Size::new(500.0, 800.0))
    .centered()
    .subscription(TodosApp::subscription)
    .run()
```

---

## 🎨 Étape 2: Refactor Design Visuel (window.dampen)

### 2.1 Nouveau Layout

**Structure souhaitée:**
```xml
<column width="fill" height="fill" padding="40">
    <scrollable>
        <container max_width="800" align_x="center">
            <column spacing="20">
                <!-- Title -->
                <text value="todos" size="100" width="fill" align_x="center" />

                <!-- Input for new tasks -->
                <text_input
                    id="new-task"
                    value="{input_value}"
                    on_input="input_changed"
                    on_submit="create_task"
                    placeholder="What needs to be done?"
                    padding="15"
                    size="30"
                />

                <!-- Controls row -->
                <row spacing="20">
                    <text value="{tasks_left_text}" width="fill" />

                    <row spacing="10">
                        <button label="All" on_click="filter:All" />
                        <button label="Active" on_click="filter:Active" />
                        <button label="Completed" on_click="filter:Completed" />
                    </row>
                </row>

                <!-- Tasks list -->
                <for each="task" in="{filtered_tasks}">
                    <!-- Task item with conditional rendering -->
                    <if test="{task.state == 'Editing'}">
                        <!-- Editing mode -->
                        <row spacing="20">
                            <text_input
                                id="task-{task.id}"
                                value="{task.edit_text}"
                                on_input="update_edit_text:{task.id}"
                                on_submit="save_edit"
                                padding="10"
                            />
                            <button label="🗑️" on_click="delete_task:{task.id}" />
                        </row>
                    </if>

                    <if test="{task.state == 'Idle'}">
                        <!-- Idle mode -->
                        <row spacing="20" align_y="center">
                            <checkbox
                                checked="{task.completed}"
                                on_change="toggle_task:{task.id}"
                                width="fill"
                                label="{task.description}"
                            />
                            <button label="✏️" on_click="edit_task:{task.id}" />
                        </row>
                    </if>
                </for>

                <!-- Empty state -->
                <if test="{filtered_tasks_len == 0}">
                    <container height="200" width="fill" align_x="center" align_y="center">
                        <text
                            value="{empty_message}"
                            size="25"
                            align_x="center"
                        />
                    </container>
                </if>
            </column>
        </container>
    </scrollable>
</column>
```

### 2.2 Styles Simples

**Supprimer** le glassmorphism complexe. Utiliser:
- Fond simple: `#1a1b1e` ou `transparent`
- Input: `padding="15"`, `size="30"`, `align_x="center"`
- Boutons filtres:
  - Actif: utiliser `style="primary"` (ou créer custom class)
  - Inactif: utiliser `style="text"`
- Texte: `color` basé sur palette Iced

### 2.3 Emojis pour Icônes

Utiliser des emojis Unicode:
- ✏️ pour éditer
- 🗑️ pour supprimer
- Pas besoin de font personnalisée

---

## 💾 Étape 3: Refactor Modèle (window.rs)

### 3.1 Définitions de Types

```rust
use uuid::Uuid;
use dampen_core::{BindingValue, HandlerRegistry, ToBindingValue};
use dampen_macros::{UiModel, dampen_ui, inventory_handlers, ui_handler};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Filter {
    #[default]
    All,
    Active,
    Completed,
}

impl Filter {
    pub fn as_str(&self) -> &str {
        match self {
            Filter::All => "All",
            Filter::Active => "Active",
            Filter::Completed => "Completed",
        }
    }
}

impl ToBindingValue for Filter {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::String(self.as_str().to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TaskState {
    #[default]
    Idle,
    Editing,
}

impl ToBindingValue for TaskState {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::String(match self {
            TaskState::Idle => "Idle".to_string(),
            TaskState::Editing => "Editing".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub completed: bool,
    #[serde(skip)]
    pub state: TaskState,
}

impl ToBindingValue for Task {
    fn to_binding_value(&self) -> BindingValue {
        let mut map = std::collections::HashMap::new();
        map.insert("id".to_string(), BindingValue::String(self.id.to_string()));
        map.insert("description".to_string(), BindingValue::String(self.description.clone()));
        map.insert("completed".to_string(), BindingValue::Bool(self.completed));
        map.insert("state".to_string(), self.state.to_binding_value());
        BindingValue::Object(map)
    }
}
```

### 3.2 Modèle Principal

```rust
#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub input_value: String,
    pub filter: Filter,
    pub tasks: Vec<Task>,
    #[ui_skip]
    pub editing_id: Option<Uuid>,
    pub edit_text: String,
    #[ui_skip]
    pub filtered_tasks: Vec<Task>,
    pub tasks_left: i64,
    pub tasks_left_text: String,
    pub empty_message: String,
    pub filtered_tasks_len: i64,
}

impl Task {
    pub fn new(description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            completed: false,
            state: TaskState::Idle,
        }
    }
}
```

### 3.3 Mise à jour des Champs Calculés

```rust
fn update_computed_fields(model: &mut Model) {
    let tasks_left = model.tasks.iter().filter(|t| !t.completed).count();
    model.tasks_left = tasks_left as i64;
    model.tasks_left_text = format!(
        "{} {} left",
        tasks_left,
        if tasks_left == 1 { "task" } else { "tasks" }
    );

    model.filtered_tasks = model.tasks
        .iter()
        .filter(|task| {
            match model.filter {
                Filter::All => true,
                Filter::Active => !task.completed,
                Filter::Completed => task.completed,
            }
        })
        .cloned()
        .collect();

    model.filtered_tasks_len = model.filtered_tasks.len() as i64;
    model.empty_message = match model.filter {
        Filter::All => "You have not created a task yet...",
        Filter::Active => "All your tasks are done! :D",
        Filter::Completed => "You have not completed a task yet...",
    }
    .to_string();
}
```

---

## ⚙️ Étape 4: Refactor Handlers (window.rs)

### 4.1 Handlers Principaux

```rust
#[ui_handler]
pub fn input_changed(model: &mut Model, value: String) {
    model.input_value = value;
}

#[ui_handler]
pub fn create_task(model: &mut Model) {
    if !model.input_value.trim().is_empty() {
        model.tasks.push(Task::new(model.input_value.clone()));
        model.input_value.clear();
        update_computed_fields(model);
    }
}

#[ui_handler]
pub fn toggle_task(model: &mut Model, id: String) {
    if let Ok(uuid) = Uuid::parse_str(&id) {
        if let Some(task) = model.tasks.iter_mut().find(|t| t.id == uuid) {
            task.completed = !task.completed;
            update_computed_fields(model);
        }
    }
}

#[ui_handler]
pub fn edit_task(model: &mut Model, id: String) {
    if let Ok(uuid) = Uuid::parse_str(&id) {
        if let Some(task) = model.tasks.iter_mut().find(|t| t.id == uuid) {
            task.state = TaskState::Editing;
            model.editing_id = Some(uuid);
            model.edit_text = task.description.clone();
        }
    }
}

#[ui_handler]
pub fn save_edit(model: &mut Model) {
    if let Some(id) = model.editing_id {
        if let Some(task) = model.tasks.iter_mut().find(|t| t.id == id) {
            if !model.edit_text.trim().is_empty() {
                task.description = model.edit_text.clone();
            }
            task.state = TaskState::Idle;
        }
        model.editing_id = None;
        model.edit_text.clear();
        update_computed_fields(model);
    }
}

#[ui_handler]
pub fn cancel_edit(model: &mut Model) {
    if let Some(id) = model.editing_id {
        if let Some(task) = model.tasks.iter_mut().find(|t| t.id == id) {
            task.state = TaskState::Idle;
        }
        model.editing_id = None;
        model.edit_text.clear();
    }
}

#[ui_handler]
pub fn update_edit_text(model: &mut Model, value: String) {
    model.edit_text = value;
}

#[ui_handler]
pub fn delete_task(model: &mut Model, id: String) {
    if let Ok(uuid) = Uuid::parse_str(&id) {
        model.tasks.retain(|t| t.id != uuid);
        update_computed_fields(model);
    }
}

#[ui_handler]
pub fn filter_changed(model: &mut Model, value: String) {
    model.filter = match value.as_str() {
        "Active" => Filter::Active,
        "Completed" => Filter::Completed,
        _ => Filter::All,
    };
    update_computed_fields(model);
}
```

### 4.2 Inventory et Registry

```rust
inventory_handlers! {
    input_changed,
    create_task,
    toggle_task,
    edit_task,
    save_edit,
    cancel_edit,
    update_edit_text,
    delete_task,
    filter_changed
}

pub fn create_app_state() -> dampen_core::AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    let mut state = dampen_core::AppState::with_handlers(document, handler_registry);
    update_computed_fields(&mut state.model);
    state
}

pub fn create_handler_registry() -> HandlerRegistry {
    use std::any::Any;
    let registry = HandlerRegistry::new();

    // Register simple handlers
    registry.register_simple("create_task", |model: &mut dyn Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            create_task(m);
        }
    });

    registry.register_simple("save_edit", |model: &mut dyn Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            save_edit(m);
        }
    });

    registry.register_simple("cancel_edit", |model: &mut dyn Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            cancel_edit(m);
        }
    });

    // Register handlers with String values
    for (name, handler) in [
        ("input_changed", input_changed as fn(&mut Model, String)),
        ("toggle_task", toggle_task as fn(&mut Model, String)),
        ("edit_task", edit_task as fn(&mut Model, String)),
        ("update_edit_text", update_edit_text as fn(&mut Model, String)),
        ("delete_task", delete_task as fn(&mut Model, String)),
        ("filter_changed", filter_changed as fn(&mut Model, String)),
    ] {
        registry.register_with_value(name, |model: &mut dyn Any, value: Box<dyn Any>| {
            if let Some(m) = model.downcast_mut::<Model>()
                && let Ok(s) = value.downcast::<String>()
            {
                handler(m, *s);
            }
        });
    }

    registry
}
```

---

## 📦 Étape 5: Simplifier SharedState (shared.rs)

### 5.1 Nouvelle Structure Simplifiée

```rust
use dampen_macros::UiModel;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize, UiModel)]
pub struct SharedState {
    pub total_tasks: i64,
    pub completed_tasks: i64,
    pub pending_tasks: i64,
    pub completion_percentage: i64,
}
```

### 5.2 Méthodes de Mise à Jour

```rust
impl SharedState {
    pub fn update_from_tasks(tasks: &[super::ui::window::Task]) -> Self {
        let total = tasks.len() as i64;
        let completed = tasks.iter().filter(|t| t.completed).count() as i64;
        let pending = total - completed;
        let completion_percentage = if total > 0 {
            (completed * 100) / total
        } else {
            0
        };

        Self {
            total_tasks: total,
            completed_tasks: completed,
            pending_tasks: pending,
            completion_percentage,
        }
    }
}
```

**Note:** Puisque pas de persistance ni de multi-view, SharedState peut être gardé mais pas activement utilisé. Il sert de placeholder pour usage futur.

---

## 🧪 Étape 6: Tests et Validation

### 6.1 Test Interpreted Mode

```bash
cd examples/todo-app
dampen run
```

**Tests manuels:**
- ✅ Créer une tâche
- ✅ Toggle completed
- ✅ Editer une tâche (inline)
- ✅ Supprimer une tâche
- ✅ Filtrer (All/Active/Completed)
- ✅ Hot-reload (modifier XML pendant exécution)

### 6.2 Test Codegen Mode

```bash
cargo build --release --features codegen
./target/release/todo-app
```

**Tests manuels:**
- ✅ Same functionality as interpreted mode
- ✅ Performance OK (no lag)
- ✅ No console errors

### 6.3 Vérifier Clippy et Formatting

```bash
cd examples/todo-app
cargo clippy -- -D warnings
cargo fmt --all
cargo test
```

---

## 📝 Checklist Finale

### Avant d'implémenter:
- [x] Créer le document de plan
- [ ] Confirmer le plan avec l'utilisateur

### Pendant l'implémentation:
- [ ] Supprimer add_task.*
- [ ] Supprimer statistics.*
- [ ] Refactor main.rs (single view)
- [ ] Refactor shared.rs (simplifier)
- [ ] Refactor window.rs (nouveau modèle + handlers)
- [ ] Refactor window.dampen (design Iced)
- [ ] Ajouter subscription clavier
- [ ] Tester interpreted mode
- [ ] Tester codegen mode
- [ ] Linting (clippy)
- [ ] Formatting (cargo fmt)

### Après l'implémentation:
- [ ] Documenter changements (si nécessaire)
- [ ] Mettre à jour AGENTS.md (si besoin)

---

## ❓ Questions pour l'Utilisateur

1. **Confirmation**: Êtes-vous OK avec:
   - Supprimer complètement `add_task` et `statistics` views?
   - Utiliser des emojis (✏️, 🗑️) au lieu de font d'icônes?
   - SharedState simplifié (pas de persistance, pas de multi-view)?

2. **Taille de fenêtre**: La fenêtre originale Iced est `500x800`. Voulez-vous:
   - Garder `500x800` (comme Iced)
   - Ou utiliser une taille différente?

3. **Tests**: Dois-je:
   - Ajouter des tests unitaires dans `window.rs`?
   - Ou me concentrer uniquement sur les tests manuels?

---

## ⚠️ Risques et Mitigations

| Risque | Impact | Mitigation |
|--------|--------|------------|
| Codegen ne marche pas | Échec en prod | Tester codegen tôt, vérifier `hello-world` |
| Hot-reload casse | Dev difficile | Garder features hot-reload, tester souvent |
| Bindings XML incorrects | UI affiche mal | Vérifier tous les `{variable}` dans XML |
| Handlers mal enregistrés | Crash | Tester chaque handler manuellement |
| Uuid parsing fails | Crash | Error handling avec `Result` |

---

## 📚 Références

- [Iced Todos Example](https://github.com/iced-rs/iced/tree/master/examples/todos)
- [Dampen Hello-World Example](../hello-world)
- [Dampen Codegen Documentation](../../crates/dampen-core/src/codegen/)

---

**Document créé le:** 2026-01-22
**Version:** 1.0
**Statut:** Plan complet, en attente de validation
