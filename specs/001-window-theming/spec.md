# Feature Specification: Window Theming

**Feature Branch**: `001-window-theming`  
**Created**: 2026-01-16  
**Status**: Draft  
**Input**: User description: "Window Theming - Customizable window appearance. Il faut ajouter une gestion complète des thèmes dans les applications Dampen."

## Clarifications

### Session 2026-01-16

- Q: Theme file location and scope → A: Global theme defined in `src/ui/theme/theme.dampen`, applied to all `.dampen` windows in the application.
- Q: Backward compatibility behavior → A: If `theme.dampen` file is absent, application behaves as current (no theming applied). Existing functionality must not break.
- Q: Dual-mode support → A: Theme system must be fully compatible with both "codegen" and "hot-reload" modes.
- Q: Theme property scope → A: Match Iced's Theme properties exactly for full framework compatibility.
- Q: Default theme when no preference set → A: Follow system dark/light preference, fallback to "light" if unavailable.
- Q: Runtime theme switching API → A: Both binding expressions (e.g., `theme="{model.current_theme}"`) and handler actions (e.g., `on_click="set_theme('dark')"`) supported.
- Q: Custom theme definition syntax → A: XML format matching `.dampen` syntax with `<themes>`, `<theme>`, `<palette>`, `<typography>`, `<spacing>` elements and `<default_theme>` selector.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Apply a Built-in Theme (Priority: P1)

As a Dampen application developer, I want to apply a pre-defined theme to my application so that I can quickly give it a professional, consistent appearance without manual styling.

**Why this priority**: This is the foundational capability that enables theme usage. Without the ability to apply themes, no other theming features are useful. Provides immediate value with minimal effort.

**Independent Test**: Can be fully tested by creating a `src/ui/theme/theme.dampen` file with a built-in theme selection and verifying all `.dampen` windows adopt the theme colors and styles.

**Acceptance Scenarios**:

1. **Given** a Dampen application with default styling, **When** a developer creates `src/ui/theme/theme.dampen` with a built-in theme (e.g., "dark" or "light"), **Then** all widgets across all `.dampen` windows display with colors and styles matching that theme.
2. **Given** an application with a theme applied, **When** the application is restarted, **Then** the theme persists and is applied on startup.
3. **Given** a theme is applied, **When** new widgets are added to any `.dampen` file, **Then** those widgets automatically inherit the current theme styling.
4. **Given** an application without a `src/ui/theme/theme.dampen` file, **When** the application runs, **Then** it behaves exactly as current Dampen applications (no theming, backward compatible).

---

### User Story 2 - Switch Themes at Runtime (Priority: P2)

As an end user of a Dampen application, I want to switch between available themes while the application is running so that I can customize the appearance to my preference without restarting.

**Why this priority**: Runtime theme switching is essential for user experience and allows applications to offer theme preferences. Builds on P1 but adds dynamic capability.

**Independent Test**: Can be tested by providing a theme toggle control in the UI and verifying all visible widgets across all windows update immediately when the theme changes.

**Acceptance Scenarios**:

1. **Given** an application with multiple themes available, **When** the user triggers a theme change, **Then** all visible widgets in all windows update their appearance within 200ms without flickering or layout shifts.
2. **Given** the user switches themes, **When** they navigate to a different window or view, **Then** the new view also displays with the selected theme.
3. **Given** a theme is selected during runtime, **When** the application preference is set to remember theme choice, **Then** the selected theme is preserved for the next session.

---

### User Story 3 - Create Custom Themes (Priority: P3)

As a Dampen application developer, I want to define custom themes with my own color palette and style properties so that I can match my application's branding requirements.

**Why this priority**: Custom themes enable brand differentiation and specialized use cases. Important for production applications but not required for basic functionality.

**Independent Test**: Can be tested by defining a custom theme in `src/ui/theme/theme.dampen` with specific colors and verifying widgets display those exact colors when the theme is applied.

**Acceptance Scenarios**:

1. **Given** I define a custom theme with specific colors in `theme.dampen`, **When** I apply that theme to my application, **Then** all widgets use the colors I specified.
2. **Given** I create a custom theme extending a built-in theme, **When** I only override certain properties, **Then** the remaining properties inherit from the base theme.
3. **Given** a custom theme definition, **When** the definition contains invalid values, **Then** the system provides a clear error message indicating which properties are invalid.

---

### User Story 4 - Widget-Level Theme Overrides (Priority: P4)

As a Dampen application developer, I want to override theme properties for specific widgets so that I can highlight important elements or create visual hierarchy while maintaining overall theme consistency.

**Why this priority**: Enables fine-grained control for advanced use cases. Most applications can function without this, but it adds flexibility for sophisticated UIs.

**Independent Test**: Can be tested by applying a local style override to a single widget and verifying only that widget differs from the theme while others remain consistent.

**Acceptance Scenarios**:

1. **Given** a themed application, **When** I apply a style override to a specific button, **Then** only that button displays with the override styles while other buttons follow the theme.
2. **Given** a widget with overrides, **When** the global theme changes, **Then** the override properties remain while non-overridden properties update to the new theme.

---

### User Story 5 - Hot-Reload Theme Changes (Priority: P2)

As a Dampen application developer using hot-reload mode, I want theme changes to be reflected immediately without restarting the application so that I can iterate quickly on the visual design.

**Why this priority**: Hot-reload is a core Dampen development feature. Theme support must integrate seamlessly with this workflow.

**Independent Test**: Can be tested by modifying `theme.dampen` while the app runs in `dampen run` mode and verifying the UI updates automatically.

**Acceptance Scenarios**:

1. **Given** an application running in hot-reload mode (`dampen run`), **When** I modify `src/ui/theme/theme.dampen`, **Then** all windows update to reflect the new theme within 500ms.
2. **Given** hot-reload is active, **When** I add or remove properties from the theme, **Then** widgets respond appropriately without crashing.

---

### Edge Cases

- What happens when an invalid theme name is specified? The system should fall back to the default theme and log a warning.
- How does the system handle theme switching during ongoing animations? Animations should complete with the new theme colors, blending smoothly.
- What happens when a custom theme is missing required properties? The system should inherit missing properties from a default base theme.
- How does theming interact with system-level dark/light mode preferences? The application should optionally respect system preferences with a configurable auto-detect mode.
- What happens if `theme.dampen` has syntax errors? The system should report clear errors and fall back to default styling (no crash).
- How does codegen mode handle themes? The theme file is compiled into the generated Rust code at build time.

## Constraints

### Backward Compatibility

- **C-001**: Adding theme support MUST NOT break any existing Dampen application that does not use theming.
- **C-002**: Applications without a `src/ui/theme/theme.dampen` file MUST behave identically to current Dampen behavior.
- **C-003**: Existing `.dampen` files MUST NOT require modification to work with or without theming.

### Dual-Mode Compatibility

- **C-004**: Theme system MUST work fully in interpreted mode (`dampen run` with hot-reload).
- **C-005**: Theme system MUST work fully in codegen mode (`dampen build` / `dampen release`).
- **C-006**: Theme hot-reload MUST be supported when running in interpreted mode.

### File Structure

- **C-007**: Theme configuration MUST be defined in `src/ui/theme/theme.dampen`.
- **C-008**: Theme applies globally to ALL `.dampen` window files in the application.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide at least two built-in themes: "light" and "dark".
- **FR-002**: System MUST load theme configuration from `src/ui/theme/theme.dampen` if present.
- **FR-003**: System MUST propagate theme properties to all child widgets across all `.dampen` windows automatically.
- **FR-004**: System MUST support runtime theme switching without application restart via both binding expressions (e.g., `theme="{model.current_theme}"`) and handler actions (e.g., `on_click="set_theme('dark')"`).
- **FR-005**: Users MUST be able to define custom themes via the `theme.dampen` file using XML format with `<themes>`, `<theme>`, `<palette>`, `<typography>`, `<spacing>` elements and `<default_theme>` selector.
- **FR-006**: Custom themes MUST be able to extend built-in themes, inheriting unspecified properties.
- **FR-007**: System MUST allow widget-level style overrides that take precedence over theme defaults.
- **FR-008**: System MUST validate theme definitions and provide clear error messages for invalid configurations.
- **FR-009**: System MUST ensure theme changes propagate to all visible widgets within 200ms.
- **FR-010**: System MUST support optional persistence of user theme preferences between sessions.
- **FR-011**: System MUST detect system-level color scheme preference (dark/light mode) and use it as the default theme selection, falling back to "light" if detection is unavailable.
- **FR-012**: System MUST support theme hot-reload in interpreted mode (`dampen run`).
- **FR-013**: System MUST compile themes into generated code in codegen mode (`dampen build`/`dampen release`).
- **FR-014**: System MUST gracefully handle missing `theme.dampen` file by using default/no-theme behavior.

### Key Entities

- **Theme**: A named collection of style properties that define the visual appearance of an application. Properties match Iced's Theme structure exactly (Palette colors, Extended palette, component-specific styles). Defined in `src/ui/theme/theme.dampen`.
- **ThemeProperty**: An individual style attribute within a theme (e.g., background color, text color, border radius). Properties can be inherited from parent themes or overridden.
- **ThemeContext**: The runtime state that tracks the currently active theme and provides theme values to widgets. Enables reactive updates when themes change. Shared across all windows.
- **StyleOverride**: Widget-specific style modifications that take precedence over theme defaults while allowing other properties to inherit from the theme.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can apply a built-in theme to an application in under 1 minute by creating a single `theme.dampen` file.
- **SC-002**: Theme switching completes and all visible widgets update within 200ms with no visible flickering.
- **SC-003**: Custom themes can be defined with 10 or fewer configuration properties for common use cases.
- **SC-004**: 100% of built-in widgets support theming (no unstyled or hard-coded appearance).
- **SC-005**: Theme validation catches and reports 100% of invalid color values, missing required properties, and syntax errors before runtime.
- **SC-006**: Applications can offer end-users theme preferences that persist correctly across 100% of app restarts.
- **SC-007**: Developers report theme implementation requires less than 30 minutes for a typical application with custom branding.
- **SC-008**: 100% of existing Dampen applications (without `theme.dampen`) continue to work without modification.
- **SC-009**: Theme hot-reload updates the UI within 500ms of file save in interpreted mode.
- **SC-010**: Codegen-built applications have identical theme behavior to interpreted mode.

## Assumptions

- The Dampen framework already has a styling system that can be extended for theming.
- Built-in themes (light/dark) follow standard accessibility contrast guidelines.
- Theme persistence uses the application's existing preference/storage mechanism if available.
- System color scheme detection relies on the underlying GUI framework's (Iced) capabilities.
- Theme definitions use a declarative format consistent with Dampen's XML-based approach.
- The `src/ui/theme/` directory may need to be created if it doesn't exist.
