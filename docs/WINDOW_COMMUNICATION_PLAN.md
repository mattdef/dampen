# Plan d'Implémentation: Inter-Window Communication (v0.2.4)

**Objectif**: Permettre la communication entre vues/fenêtres dans les applications Dampen multi-vues.

**Date**: 2026-01-14  
**Statut**: Analyse

---

## Table des Matières

1. [Résumé Exécutif](#résumé-exécutif)
2. [Architecture Actuelle](#architecture-actuelle)
3. [Analyse des Besoins](#analyse-des-besoins)
4. [Options de Design](#options-de-design)
5. [Design Recommandé](#design-recommandé)
6. [Plan d'Implémentation](#plan-dimplémentation)
7. [Impact sur l'Existant](#impact-sur-lexistant)
8. [Compatibilité Modes](#compatibilité-modes)
9. [Tests](#tests)
10. [Migration](#migration)

---

## Résumé Exécutif

### Problème

Actuellement, les vues Dampen sont **complètement isolées**:
- Chaque vue a son propre `Model` indépendant
- Chaque vue a son propre `HandlerRegistry`
- Aucun mécanisme pour partager des données entre vues
- Aucun moyen d'envoyer des messages entre vues

### Solution Proposée

Implémenter un **système de communication inter-vues** basé sur:
1. **État Partagé** (`SharedState`) - Données accessibles depuis toutes les vues
2. **Bus de Messages** (`MessageBus`) - Communication événementielle entre vues
3. **Nouveaux attributs XML** - Syntaxe déclarative pour les bindings partagés

### Compatibilité

- ✅ 100% rétro-compatible (opt-in uniquement)
- ✅ Fonctionne en mode **interprété** (hot-reload)
- ✅ Fonctionne en mode **codegen** (production)
- ✅ Préserve l'isolation des vues par défaut

---

## Architecture Actuelle

### Structure Multi-Vues

```
┌─────────────────────────────────────────────────────────────────┐
│                         SettingsApp                              │
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │  current_view: CurrentView                                   │ │
│  ├─────────────────────────────────────────────────────────────┤ │
│  │  window_state: AppState<window::Model>                       │ │
│  │  ├── document: DampenDocument                                │ │
│  │  ├── model: window::Model          ← État ISOLÉ              │ │
│  │  └── handler_registry: HandlerRegistry                       │ │
│  ├─────────────────────────────────────────────────────────────┤ │
│  │  settings_state: AppState<settings::Model>                   │ │
│  │  ├── document: DampenDocument                                │ │
│  │  ├── model: settings::Model        ← État ISOLÉ              │ │
│  │  └── handler_registry: HandlerRegistry                       │ │
│  └─────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Flux de Messages Actuel

```
Widget Click → HandlerMessage::Handler(name, value)
            → Message::Handler(HandlerMessage)
            → update() match self.current_view
            → dispatch_handler_with_task()
            → Vue active uniquement
```

### Code Généré par `#[dampen_app]`

```rust
// Enum de navigation
pub enum CurrentView { Window, Settings }

// Application générée
pub struct SettingsApp {
    window_state: AppState<window::Model>,
    settings_state: AppState<settings::Model>,
    current_view: CurrentView,
    #[cfg(debug_assertions)]
    error_overlay: ErrorOverlay,
}
```

### Limitations Identifiées

| Aspect | État Actuel | Besoin |
|--------|-------------|--------|
| Partage de données | Impossible | Données globales accessibles |
| Communication | View switch uniquement | Messages entre vues |
| Synchronisation | N/A | État partagé synchronisé |
| Bindings XML | `{model.field}` uniquement | `{shared.field}` aussi |

---

## Analyse des Besoins

### Cas d'Usage Principaux

1. **Préférences Utilisateur**
   - Vue "Settings" modifie le thème
   - Vue "Main" doit refléter le changement immédiatement

2. **Session Utilisateur**
   - Données utilisateur (nom, rôle) partagées entre toutes les vues
   - Pas de duplication de l'état

3. **Notifications Cross-View**
   - Une vue déclenche une action
   - Autre(s) vue(s) réagissent

4. **Panier d'Achat (e-commerce)**
   - Produits ajoutés depuis plusieurs vues
   - Compteur affiché sur toutes les vues

### Exigences Non-Fonctionnelles

| Exigence | Description |
|----------|-------------|
| Performance | < 1ms pour accès état partagé |
| Thread Safety | Safe pour accès concurrent |
| Hot-Reload | État partagé préservé au reload |
| Type Safety | Erreurs de compilation si types incorrects |
| Simplicité | API intuitive, syntaxe XML naturelle |

---

## Options de Design

### Option A: État Partagé Centralisé

```rust
// Nouveau trait dans dampen-core
pub trait SharedModel: UiBindable + Send + Sync + 'static {}

// Application avec état partagé
pub struct MyApp {
    window_state: AppState<window::Model>,
    settings_state: AppState<settings::Model>,
    shared: Arc<RwLock<SharedState>>,  // ← Nouveau
    current_view: CurrentView,
}

// Accès depuis handlers
registry.register_with_shared("update_theme", |model, shared| {
    shared.theme = model.selected_theme.clone();
});
```

**Avantages**:
- Simple à comprendre
- Performance optimale (accès direct)
- Typage fort

**Inconvénients**:
- Nécessite définition explicite de `SharedState`
- Couplage entre vues via le type partagé

### Option B: Bus de Messages

```rust
// Nouveau type de message
pub enum InterViewMessage {
    ThemeChanged(String),
    UserLoggedIn(User),
    CartUpdated(Cart),
}

// Souscription depuis une vue
registry.subscribe("ThemeChanged", |model, theme: String| {
    model.current_theme = theme;
});

// Émission depuis une autre vue
registry.emit(InterViewMessage::ThemeChanged("dark".into()));
```

**Avantages**:
- Découplage total entre vues
- Pattern pub/sub familier
- Extensible

**Inconvénients**:
- Pas de typage fort cross-view
- Plus complexe pour données simples
- Synchronisation manuelle

### Option C: Hybrid (État + Messages) - **RECOMMANDÉ**

Combiner les deux approches:
1. **État Partagé** pour données stables (user, preferences, cart)
2. **Messages** pour événements ponctuels (notifications, actions)

```rust
// Configuration dans #[dampen_app]
#[dampen_app(
    ui_dir = "src/ui",
    shared_model = "SharedState",  // ← Nouveau
    // ...
)]
struct MyApp;

// État partagé défini par l'utilisateur
#[derive(Default, UiModel)]
pub struct SharedState {
    pub user: Option<User>,
    pub theme: String,
    pub cart_count: u32,
}

// Dans XML: accès via préfixe "shared."
<text value="{shared.user.name}" />
<text value="Panier: {shared.cart_count}" />
```

---

## Design Recommandé

### Vue d'Ensemble Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         MyApp (généré)                           │
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │         SharedContext<SharedState>                           │ │
│  │  ┌─────────────────────────────────────────────────────┐    │ │
│  │  │  state: Arc<RwLock<SharedState>>                    │    │ │
│  │  │  subscribers: HashMap<String, Vec<Callback>>        │    │ │
│  │  │  pending_messages: VecDeque<InterViewMessage>       │    │ │
│  │  └─────────────────────────────────────────────────────┘    │ │
│  └─────────────────────────────────────────────────────────────┘ │
│                              ↑                                    │
│                    (référence partagée)                          │
│                              ↓                                    │
│  ┌────────────────────┐         ┌────────────────────┐          │
│  │  window_state      │         │  settings_state    │          │
│  │  ├── model         │         │  ├── model         │          │
│  │  ├── handlers      │         │  ├── handlers      │          │
│  │  └── shared_ctx ───┼─────────┼──┴── shared_ctx    │          │
│  └────────────────────┘         └────────────────────┘          │
└─────────────────────────────────────────────────────────────────┘
```

### Nouveaux Types (dampen-core)

```rust
// crates/dampen-core/src/shared/mod.rs

use std::sync::{Arc, RwLock};
use std::collections::HashMap;

/// Contexte partagé entre vues
pub struct SharedContext<S: UiBindable + Send + Sync + 'static> {
    /// État partagé protégé par RwLock
    state: Arc<RwLock<S>>,
    
    /// Messages inter-vues en attente
    pending_messages: Arc<RwLock<Vec<Box<dyn std::any::Any + Send>>>>,
}

impl<S: UiBindable + Send + Sync + 'static> SharedContext<S> {
    pub fn new(initial: S) -> Self {
        Self {
            state: Arc::new(RwLock::new(initial)),
            pending_messages: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Lecture de l'état partagé (binding XML)
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, S> {
        self.state.read().expect("SharedContext lock poisoned")
    }
    
    /// Modification de l'état partagé (handler)
    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, S> {
        self.state.write().expect("SharedContext lock poisoned")
    }
    
    /// Émettre un message inter-vue
    pub fn emit<M: Send + 'static>(&self, message: M) {
        if let Ok(mut pending) = self.pending_messages.write() {
            pending.push(Box::new(message));
        }
    }
    
    /// Drainer les messages en attente
    pub fn drain_messages(&self) -> Vec<Box<dyn std::any::Any + Send>> {
        if let Ok(mut pending) = self.pending_messages.write() {
            std::mem::take(&mut *pending)
        } else {
            Vec::new()
        }
    }
}

impl<S: UiBindable + Send + Sync + 'static> Clone for SharedContext<S> {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
            pending_messages: Arc::clone(&self.pending_messages),
        }
    }
}
```

### Extension de HandlerRegistry

```rust
// crates/dampen-core/src/handler/mod.rs

pub enum HandlerEntry {
    Simple(Arc<dyn Fn(&mut dyn Any) + Send + Sync>),
    WithValue(Arc<dyn Fn(&mut dyn Any, Box<dyn Any>) + Send + Sync>),
    WithCommand(Arc<dyn Fn(&mut dyn Any) -> Box<dyn Any> + Send + Sync>),
    
    // NOUVEAU: Handler avec accès au contexte partagé
    WithShared(Arc<dyn Fn(&mut dyn Any, &dyn Any) + Send + Sync>),
    
    // NOUVEAU: Handler avec value + shared
    WithValueAndShared(Arc<dyn Fn(&mut dyn Any, Box<dyn Any>, &dyn Any) + Send + Sync>),
    
    // NOUVEAU: Handler avec command + shared
    WithCommandAndShared(Arc<dyn Fn(&mut dyn Any, &dyn Any) -> Box<dyn Any> + Send + Sync>),
}

impl HandlerRegistry {
    /// Register handler avec accès shared context
    pub fn register_with_shared<F>(&self, name: &str, handler: F)
    where
        F: Fn(&mut dyn Any, &dyn Any) + Send + Sync + 'static,
    {
        if let Ok(mut handlers) = self.handlers.write() {
            handlers.insert(name.to_string(), HandlerEntry::WithShared(Arc::new(handler)));
        }
    }
    
    /// Dispatch avec contexte partagé
    pub fn dispatch_with_shared(
        &self,
        handler_name: &str,
        model: &mut dyn Any,
        shared: &dyn Any,
        value: Option<String>,
    ) -> Option<Box<dyn Any>> {
        if let Some(entry) = self.get(handler_name) {
            match entry {
                // Handlers existants (rétro-compatibilité)
                HandlerEntry::Simple(h) => { h(model); None }
                HandlerEntry::WithValue(h) => {
                    h(model, Box::new(value.unwrap_or_default()));
                    None
                }
                HandlerEntry::WithCommand(h) => Some(h(model)),
                
                // Nouveaux handlers
                HandlerEntry::WithShared(h) => { h(model, shared); None }
                HandlerEntry::WithValueAndShared(h) => {
                    h(model, Box::new(value.unwrap_or_default()), shared);
                    None
                }
                HandlerEntry::WithCommandAndShared(h) => Some(h(model, shared)),
            }
        } else {
            None
        }
    }
}
```

### Extension de AppState

```rust
// crates/dampen-core/src/state/mod.rs

/// Application state with optional shared context
pub struct AppState<M: UiBindable = (), S: UiBindable + Send + Sync = ()> {
    pub document: DampenDocument,
    pub model: M,
    pub handler_registry: HandlerRegistry,
    
    /// NOUVEAU: Référence optionnelle au contexte partagé
    pub shared_context: Option<SharedContext<S>>,
    
    _marker: PhantomData<(M, S)>,
}

impl<M: UiBindable, S: UiBindable + Send + Sync + 'static> AppState<M, S> {
    /// Créer avec contexte partagé
    pub fn with_shared(
        document: DampenDocument,
        model: M,
        handler_registry: HandlerRegistry,
        shared_context: SharedContext<S>,
    ) -> Self {
        Self {
            document,
            model,
            handler_registry,
            shared_context: Some(shared_context),
            _marker: PhantomData,
        }
    }
    
    /// Accès en lecture au shared state pour bindings
    pub fn shared(&self) -> Option<std::sync::RwLockReadGuard<'_, S>> {
        self.shared_context.as_ref().map(|ctx| ctx.read())
    }
}
```

### Syntaxe XML

```xml
<!-- Binding sur état partagé -->
<text value="Bienvenue, {shared.user.name}!" />

<!-- Binding conditionnel -->
<button label="Se connecter" visible="{!shared.user.is_logged_in}" />

<!-- Panier avec état partagé -->
<text value="Panier ({shared.cart.items.len()})" />

<!-- Handler qui modifie l'état partagé -->
<button label="Thème Sombre" on_click="set_dark_theme" />
```

### Modifications du Macro `#[dampen_app]`

```rust
// Nouvel attribut optionnel
#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    // NOUVEAU: Type d'état partagé (optionnel)
    shared_model = "SharedState",
    // NOUVEAU: Variante pour messages inter-vues (optionnel)  
    inter_view_variant = "InterView",
    // Existants...
    hot_reload_variant = "HotReload",
    switch_view_variant = "SwitchToView",
)]
struct MyApp;
```

#### Code Généré

```rust
// Avec shared_model = "SharedState"
pub struct MyApp {
    window_state: AppState<window::Model, SharedState>,
    settings_state: AppState<settings::Model, SharedState>,
    shared_context: SharedContext<SharedState>,  // ← Nouveau
    current_view: CurrentView,
    #[cfg(debug_assertions)]
    error_overlay: ErrorOverlay,
}

impl MyApp {
    pub fn init() -> Self {
        let shared_context = SharedContext::new(SharedState::default());
        
        Self {
            window_state: window::create_app_state_with_shared(shared_context.clone()),
            settings_state: settings::create_app_state_with_shared(shared_context.clone()),
            shared_context,
            current_view: CurrentView::Window,
            #[cfg(debug_assertions)]
            error_overlay: ErrorOverlay::new(),
        }
    }
    
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Handler(handler_msg) => {
                // Dispatch avec shared context
                match self.current_view {
                    CurrentView::Window => {
                        dispatch_handler_with_shared(
                            &mut self.window_state.model,
                            &self.window_state.handler_registry,
                            &self.shared_context,
                            handler_msg,
                        )
                    }
                    // ...
                }
            }
            Message::InterView(msg) => {
                // Traiter messages inter-vues
                self.handle_inter_view_message(msg)
            }
            // ...
        }
    }
}
```

### Extension de DampenWidgetBuilder

```rust
// crates/dampen-iced/src/builder.rs

impl<'a, M: UiBindable, S: UiBindable> DampenWidgetBuilder<'a, M, S> {
    /// Évalue un binding, cherchant d'abord dans model puis dans shared
    fn evaluate_binding(&self, expr: &str) -> Option<BindingValue> {
        // Préfixe "shared." → accès état partagé
        if let Some(path) = expr.strip_prefix("shared.") {
            if let Some(ref shared) = self.shared_context {
                let guard = shared.read();
                let parts: Vec<&str> = path.split('.').collect();
                return guard.get_field(&parts);
            }
        }
        
        // Sinon, accès model local
        let parts: Vec<&str> = expr.split('.').collect();
        self.model.get_field(&parts)
    }
}
```

---

## Plan d'Implémentation

### Phase 1: Infrastructure Core (2-3 jours)

| Tâche | Fichier | Description |
|-------|---------|-------------|
| T1.1 | `dampen-core/src/shared/mod.rs` | Créer `SharedContext<S>` struct |
| T1.2 | `dampen-core/src/lib.rs` | Exporter le nouveau module |
| T1.3 | `dampen-core/src/handler/mod.rs` | Ajouter `WithShared` variants |
| T1.4 | `dampen-core/src/handler/mod.rs` | Implémenter `dispatch_with_shared()` |
| T1.5 | `dampen-core/src/state/mod.rs` | Étendre `AppState<M, S>` avec paramètre générique |
| T1.6 | Tests unitaires | Tests pour SharedContext et handlers |

### Phase 2: Extension Macro (2-3 jours)

| Tâche | Fichier | Description |
|-------|---------|-------------|
| T2.1 | `dampen-macros/src/dampen_app.rs` | Parser `shared_model` attribut |
| T2.2 | `dampen-macros/src/dampen_app.rs` | Générer `SharedContext` dans struct |
| T2.3 | `dampen-macros/src/dampen_app.rs` | Modifier `init()` pour partager contexte |
| T2.4 | `dampen-macros/src/dampen_app.rs` | Modifier `update()` pour dispatch shared |
| T2.5 | `dampen-macros/src/dampen_app.rs` | Générer helper `handle_inter_view_message` |
| T2.6 | Tests macro | Tests de génération de code |

### Phase 3: Bindings XML (2-3 jours)

| Tâche | Fichier | Description |
|-------|---------|-------------|
| T3.1 | `dampen-iced/src/builder.rs` | Étendre `DampenWidgetBuilder` avec shared |
| T3.2 | `dampen-iced/src/builder.rs` | Implémenter résolution `{shared.xxx}` |
| T3.3 | `dampen-core/src/parser/` | Valider syntaxe binding shared |
| T3.4 | `dampen-core/src/codegen/` | Codegen pour bindings shared |
| T3.5 | Tests intégration | Tests binding shared end-to-end |

### Phase 4: Exemple et Documentation (1-2 jours)

| Tâche | Fichier | Description |
|-------|---------|-------------|
| T4.1 | `examples/shared-state/` | Créer exemple complet |
| T4.2 | `docs/USAGE.md` | Documenter syntaxe et API |
| T4.3 | `docs/XML_SCHEMA.md` | Documenter bindings `{shared.}` |
| T4.4 | `CHANGELOG.md` | Entrée pour v0.2.4 |

### Phase 5: Tests et Polish (1-2 jours)

| Tâche | Fichier | Description |
|-------|---------|-------------|
| T5.1 | Tests hot-reload | Vérifier préservation shared state |
| T5.2 | Tests codegen | Vérifier parité interpreted/codegen |
| T5.3 | Benchmarks | Performance accès shared state |
| T5.4 | CI | Ajouter tests nouveaux modules |

---

## Impact sur l'Existant

### Composants Modifiés

| Composant | Modification | Risque | Migration |
|-----------|--------------|--------|-----------|
| `AppState<M>` | Nouveau param générique `S` | Moyen | Type alias pour rétro-compat |
| `HandlerRegistry` | Nouveaux variants | Faible | Additif uniquement |
| `HandlerEntry` | Nouveaux variants | Faible | Enum extensible |
| `#[dampen_app]` | Nouvel attribut | Faible | Optionnel |
| `DampenWidgetBuilder` | Shared context | Moyen | Paramètre optionnel |

### Rétro-Compatibilité

```rust
// AVANT (v0.2.3) - Continue de fonctionner
pub struct AppState<M: UiBindable = ()> { ... }

// APRÈS (v0.2.4) - Type alias pour compatibilité
pub type AppState<M> = AppStateWithShared<M, ()>;
pub struct AppStateWithShared<M: UiBindable = (), S: UiBindable = ()> { ... }
```

### Exemples Existants

| Exemple | Impact | Action |
|---------|--------|--------|
| `hello-world` | Aucun | Aucune modification |
| `counter` | Aucun | Aucune modification |
| `todo-app` | Aucun | Aucune modification |
| `settings` | Optionnel | Peut ajouter shared state |

---

## Compatibilité Modes

### Mode Interprété (Hot-Reload)

```rust
// Le SharedContext survit au hot-reload
pub fn hot_reload(&mut self, new_document: DampenDocument) {
    self.document = new_document;
    // model: préservé ✓
    // shared_context: préservé ✓
}
```

**Comportement**:
- ✅ État partagé préservé lors du reload XML
- ✅ Handlers avec shared continuent de fonctionner
- ✅ Bindings `{shared.}` réévalués après reload

### Mode Codegen (Production)

```rust
// build.rs génère le code statique
fn generate_view_with_shared() -> TokenStream {
    quote! {
        pub fn view(&self) -> Element<Message> {
            // Accès au shared_context compile-time
            let theme = &self.shared_context.read().theme;
            // ...
        }
    }
}
```

**Comportement**:
- ✅ Accès shared typé statiquement
- ✅ Pas de parsing runtime
- ✅ Performance optimale

### Parité Interprété/Codegen

| Fonctionnalité | Interprété | Codegen | Parité |
|----------------|------------|---------|--------|
| Binding `{shared.x}` | Runtime eval | Compilé | ✅ |
| Handler `WithShared` | Runtime dispatch | Inline call | ✅ |
| Hot-reload shared | Préservé | N/A | ✅ |
| Type safety | Runtime | Compile-time | ✅ |

---

## Tests

### Tests Unitaires

```rust
// dampen-core/src/shared/tests.rs

#[test]
fn shared_context_read_write() {
    #[derive(Default, UiBindable)]
    struct State { count: i32 }
    
    let ctx = SharedContext::new(State { count: 0 });
    
    // Write
    ctx.write().count = 42;
    
    // Read
    assert_eq!(ctx.read().count, 42);
}

#[test]
fn shared_context_clone_shares_state() {
    let ctx1 = SharedContext::new(State { count: 0 });
    let ctx2 = ctx1.clone();
    
    ctx1.write().count = 10;
    assert_eq!(ctx2.read().count, 10);
}

#[test]
fn handler_dispatch_with_shared() {
    let registry = HandlerRegistry::new();
    registry.register_with_shared("update_shared", |model, shared| {
        let model = model.downcast_mut::<LocalModel>().unwrap();
        let shared = shared.downcast_ref::<SharedContext<GlobalState>>().unwrap();
        shared.write().value = model.local_value.clone();
    });
    
    // Test dispatch...
}
```

### Tests Intégration

```rust
// examples/shared-state/tests/integration.rs

#[test]
fn shared_state_across_views() {
    // Créer app avec shared state
    let app = SharedStateApp::init();
    
    // Modifier depuis vue 1
    app.update(Message::Handler(HandlerMessage::Handler(
        "set_theme".into(), 
        Some("dark".into())
    )));
    
    // Vérifier depuis vue 2
    assert_eq!(app.shared_context.read().theme, "dark");
}

#[test]
fn hot_reload_preserves_shared() {
    let mut app = SharedStateApp::init();
    app.shared_context.write().count = 42;
    
    // Simuler hot-reload
    let new_doc = parse(NEW_XML).unwrap();
    app.window_state.hot_reload(new_doc);
    
    // Shared state préservé
    assert_eq!(app.shared_context.read().count, 42);
}
```

### Tests Parité

```rust
// tests/parity_tests.rs

#[test]
fn parity_shared_binding_evaluation() {
    let xml = r#"<text value="{shared.user.name}" />"#;
    
    // Mode interprété
    let interpreted = interpret_with_shared(xml, &shared_state);
    
    // Mode codegen
    let compiled = compile_with_shared(xml, &shared_state);
    
    assert_eq!(interpreted.text(), compiled.text());
}
```

---

## Migration

### Guide de Migration (Optionnel)

Les applications existantes n'ont **aucune modification requise**. Pour adopter la communication inter-vues:

#### Étape 1: Définir l'État Partagé

```rust
// src/shared.rs
use dampen_macros::UiModel;
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Debug, UiModel, Serialize, Deserialize)]
pub struct SharedState {
    pub user: Option<User>,
    pub theme: String,
    pub notifications: Vec<Notification>,
}

#[derive(Clone, Debug, UiModel, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
}
```

#### Étape 2: Configurer le Macro

```rust
// src/main.rs
use dampen_macros::dampen_app;

mod shared;  // ← Nouveau module

#[derive(Clone, Debug)]
enum Message {
    Handler(HandlerMessage),
    SwitchToView(CurrentView),
    #[cfg(debug_assertions)]
    HotReload(FileEvent),
}

#[dampen_app(
    ui_dir = "src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    shared_model = "shared::SharedState",  // ← Nouveau
    // ...
)]
struct MyApp;
```

#### Étape 3: Utiliser dans les Handlers

```rust
// src/ui/settings.rs
pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();
    
    // Handler avec accès shared
    registry.register_with_shared("update_theme", |model, shared| {
        let model = model.downcast_mut::<Model>().unwrap();
        let shared = shared.downcast_ref::<SharedContext<SharedState>>().unwrap();
        
        // Modifier l'état partagé
        shared.write().theme = model.selected_theme.clone();
    });
    
    registry
}
```

#### Étape 4: Bindings XML

```xml
<!-- src/ui/window.dampen -->
<column>
    <text value="Bonjour, {shared.user.name}!" />
    <text value="Thème: {shared.theme}" />
</column>

<!-- src/ui/settings.dampen -->
<column>
    <picker 
        options="{themes}" 
        selected="{shared.theme}"
        on_change="update_theme" 
    />
</column>
```

---

## Risques et Mitigations

| Risque | Impact | Probabilité | Mitigation |
|--------|--------|-------------|------------|
| Deadlock RwLock | Haut | Faible | Timeouts, try_read/try_write |
| Performance dégradée | Moyen | Faible | Benchmarks, cache local |
| Complexité API | Moyen | Moyen | Bons exemples, docs complètes |
| Breaking changes | Haut | Très faible | Type aliases, tests exhaustifs |

---

## Estimation

| Phase | Durée | Effort |
|-------|-------|--------|
| Phase 1: Core | 2-3 jours | 16-24h |
| Phase 2: Macro | 2-3 jours | 16-24h |
| Phase 3: Bindings | 2-3 jours | 16-24h |
| Phase 4: Docs | 1-2 jours | 8-16h |
| Phase 5: Tests | 1-2 jours | 8-16h |
| **Total** | **8-13 jours** | **64-104h** |

---

## Conclusion

Ce plan d'implémentation propose une solution **hybride** combinant état partagé et messagerie inter-vues, offrant:

1. **Simplicité** - Préfixe `{shared.}` intuitif dans XML
2. **Type Safety** - Typage fort compile-time
3. **Performance** - Accès direct via `Arc<RwLock<S>>`
4. **Rétro-compatibilité** - 100% opt-in, aucun breaking change
5. **Parité modes** - Fonctionne identiquement en interprété et codegen

La prochaine étape est de créer le dossier de spécification formel `specs/003-window-communication/` et démarrer l'implémentation Phase 1.
