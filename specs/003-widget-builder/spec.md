# Feature Specification: Gravity Widget Builder

**Feature Branch**: `003-widget-builder`  
**Created**: 2026-01-03  
**Status**: Draft  
**Input**: User description: "Ajouter un système de "Widget Builder" dans gravity-iced qui simplifie radicalement l'interprétation du markup Gravity. L'objectif est d'avoir un équivalent à XAML/Avalonia où le framework interprète automatiquement le XML sans nécessiter de code boilerplate de conversion dans les exemples.

Le système doit :
1. Rester backend-agnostique (gravity-core sans dépendance UI)
2. Centraliser toute la logique d'interprétation dans gravity-iced
3. Permettre aux exemples de n'utiliser qu'une seule ligne pour afficher l'UI
4. Gérer automatiquement : bindings, événements, style, layout, enfants récursifs
5. Utiliser les implémentations From pour les conversions de types

Exemple de ce qu'on veut obtenir :
```rust
// AVANT (410 lignes de code)
fn render_text(...) { /* 30 lignes de conversions manuelles */ }
fn render_button(...) { /* conversions manuelles */ }
fn render_column(...) { /* conversions manuelles */ }
// ... et une fonction par widget

// APRÈS (10 lignes de code)
fn view(state: &AppState) -> Element<'_, Message> {
    GravityWidgetBuilder::new(&state.document.root, &state.model, &state.handler_registry)
        .build()
}
```

Architecture proposée :
- gravity-iced/src/convert.rs : Implémentations From pour IR → Iced
- gravity-iced/src/builder.rs : GravityWidgetBuilder struct avec méthode build()
- gravity-iced/src/lib.rs : Exporter le builder
- Simplifier examples/styling/src/main.rs et state_demo.rs"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Simplifier l'interprétation du markup (Priority: P1)

Un développeur veut afficher une UI Gravity avec une seule ligne de code, sans avoir à écrire de fonctions de conversion manuelles ou de dispatcher les types de widgets.

**Why this priority**: C'est le but ultime de Gravity - être aussi simple que XAML/Avalonia. Sans ça, les développeurs doivent écrire du boilerplate à chaque fois qu'ils utilisent Gravity.

**Independent Test**: Créer un exemple qui affiche une UI complexe (boutons, texte, layout) avec seulement `GravityWidgetBuilder::new(...).build()`, et vérifier que tout fonctionne (bindings, événements, styles).

**Acceptance Scenarios**:

1. **Given** un document Gravity avec un `<column>` contenant des `<text>` et `<button>`, **When** utilisé avec `GravityWidgetBuilder`, **Then** l'UI s'affiche correctement sans code de conversion
2. **Given** un widget avec `background="#3498db"` dans le XML, **When** utilisé avec le builder, **Then** la couleur s'applique automatiquement via `From` implémenté
3. **Given** un widget avec `on_click="increment"`, **When** utilisé avec le builder, **Then** l'événement est automatiquement connecté au handler
4. **Given** un widget avec `{count}` dans le texte, **When** le modèle change, **Then** le texte se met à jour automatiquement
5. **Given** des widgets imbriqués, **When** utilisé avec le builder, **Then** la récursion fonctionne et tous les enfants sont rendus

---

### User Story 2 - Centraliser la logique d'interprétation (Priority: P2)

Toute la logique pour interpréter les nœuds Gravity (style, layout, events, bindings) doit être centralisée dans `gravity-iced`, pas dupliquée dans chaque exemple.

**Why this priority**: La centralisation améliore la maintenabilité. Si Gravity ajoute un nouveau widget ou attribut, on ne modifie qu'un seul endroit.

**Independent Test**: Ajouter un nouveau type de widget dans Gravity, vérifier qu'on ne modifie que le builder, pas les exemples.

**Acceptance Scenarios**:

1. **Given** le builder centralise toute l'interprétation, **When** on ajoute un nouveau widget, **Then** on ne modifie que gravity-iced, pas les exemples
2. **Given** le builder utilise les types IR déjà parsés, **When** on lit le code, **Then** il n'y a pas de duplication de logique de parsing
3. **Given** les conversions sont dans `convert.rs`, **When** on veut supporter un nouveau backend, **Then** on ajoute juste de nouvelles implémentations `From`

---

### User Story 3 - Supporter tous les cas d'usage des exemples actuels (Priority: P1)

Le builder doit supporter tout ce que les exemples actuels font : bindings, événements, style, layout, états.

**Why this priority**: Si le builder ne supporte pas un cas d'usage, les développeurs devront contourner ou abandonner le builder, ce qui ruine l'objectif de simplicité.

**Independent Test**: Convertir `examples/styling/src/main.rs` et `examples/styling/src/state_demo.rs` pour utiliser le builder, et vérifier qu'ils fonctionnent identiquement.

**Acceptance Scenarios**:

1. **Given** un exemple avec bindings complexes `{user.name}`, **When** utilisé avec le builder, **Then** les bindings fonctionnent
2. **Given** un exemple avec des handlers `on_click`, `on_submit`, **When** utilisé avec le builder, **Then** les événements sont connectés
3. **Given** un exemple avec style inline et classes, **When** utilisé avec le builder, **Then** les styles s'appliquent correctement
4. **Given** un exemple avec layout (padding, spacing, width), **When** utilisé avec le builder, **Then** le layout est correct
5. **Given** un exemple avec états (hover, active), **When** utilisé avec le builder, **Then** les transitions d'état fonctionnent

---

### Edge Cases

- Que se passe-t-il quand un widget n'a pas de style défini dans le XML ? Le builder doit utiliser les valeurs par défaut Iced
- Comment le builder gère-t-il les widgets personnalisés non supportés ? Il doit retourner un widget vide ou un placeholder avec un message d'erreur clair
- Que se passe-t-il quand un handler n'existe pas dans le registry ? Le builder doit ignorer silencieusement ou logger un warning
- Comment le builder gère-t-il les performances avec des UIs très complexes (1000+ widgets) ? Il doit être aussi performant que l'approche manuelle actuelle
- Que se passe-t-il quand les conversions `From` échouent ? Les types IR sont validés au parsing, donc les conversions ne devraient jamais échouer

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a widget builder that accepts parsed UI document, application model, and event handler registry
- **FR-002**: System MUST provide a build method that returns a renderable UI element
- **FR-003**: System MUST automatically process all child widgets recursively
- **FR-004**: System MUST automatically evaluate data bindings in widget properties
- **FR-005**: System MUST automatically connect UI events to registered handlers
- **FR-006**: System MUST automatically apply visual styles from UI definitions
- **FR-007**: System MUST automatically apply layout constraints from UI definitions
- **FR-008**: System MUST provide automatic type conversions between UI definition types and rendering framework types
- **FR-009**: System MUST support conversions for all visual properties: colors, dimensions, spacing, borders, shadows, backgrounds, and transforms
- **FR-010**: Core parsing logic MUST remain independent of any specific rendering framework
- **FR-011**: Complete UI examples MUST be expressible in fewer than 50 lines of application code
- **FR-012**: System MUST support all common widget types: text display, buttons, vertical layouts, horizontal layouts, and containers
- **FR-013**: System MUST handle missing or incomplete UI definition attributes gracefully using appropriate defaults
- **FR-014**: System MUST provide verbose logging mode for debugging widget creation and binding evaluation
- **FR-015**: System MUST gracefully degrade when HandlerRegistry or binding evaluation is unavailable

### Key Entities

- **Widget Builder**: Central component that orchestrates widget creation from parsed UI definitions
- **Type Conversion System**: Mechanism for automatically converting between UI definition types and rendering framework types
- **Event Handler Registry**: System for managing and connecting UI events to application logic

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Existing complex UI examples can be reduced from 410+ lines to fewer than 50 lines while maintaining identical functionality
- **SC-002**: Existing stateful UI examples can be reduced from ~200 lines to fewer than 50 lines
- **SC-003**: All existing tests continue to pass without requiring modifications
- **SC-004**: New implementation passes all code quality checks without warnings
- **SC-005**: Build time for UI rendering components increases by less than 10%
- **SC-006**: Runtime rendering performance matches or exceeds manual implementation approaches (target: 50ms for 1000 widgets)
- **SC-007**: Developers can create new UI examples using fewer than 10 lines of application code
- **SC-008**: Adding new widget types requires changes only to the widget builder system, not to existing examples

---

### Assumptions

- Les implémentations `From` peuvent être définies dans `gravity-iced` sans créer de dépendance cyclique
- Le `HandlerRegistry` et `evaluate_binding_expr` sont déjà fonctionnels et peuvent être utilisés par le builder
- Les types IR (`WidgetNode`, `StyleProperties`, `LayoutConstraints`) sont suffisamment complets pour supporter le builder
- Les performances de la récursion dans `build()` sont acceptables pour des UIs typiques (< 1000 widgets)
- Les développeurs acceptent une perte de contrôle granulaire (composition manuelle et accès direct à l'API Iced) en échange de la simplicité

---

## Clarifications

### Session 2026-01-03

- Q: What are the specific performance targets for rendering? → A: 50ms for 1000 widgets
- Q: How should developers experience errors (missing handlers, unsupported widgets)? → A: Runtime error overlay in UI + console logging
- Q: What happens if HandlerRegistry or evaluate_binding_expr are missing? → A: Graceful degradation with clear error messages
- Q: Should the builder provide logging/tracing for debugging? → A: Verbose logging mode with --verbose flag
- Q: What specific control are developers giving up? → A: Manual widget composition and direct Iced API access
