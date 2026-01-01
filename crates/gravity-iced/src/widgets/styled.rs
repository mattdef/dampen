//! Styled wrapper widget for state-based styling
//!
//! This module provides a wrapper widget that applies dynamic styles based on
//! widget state (hover, focus, active, disabled).
//!
//! Note: Placeholder implementation for Phase 1. Full state-based styling
//! will be implemented in Phase 10.

/// Placeholder for styled container
/// Full implementation will be added in Phase 10
pub struct Styled;

impl Styled {
    /// Placeholder constructor
    pub fn new() -> Self {
        Self
    }
}

impl<'a, Message, ThemeType, RendererType> Styled<'a, Message, ThemeType, RendererType>
where
    ThemeType: iced::Theme,
    RendererType: iced::Renderer,
{
    /// Create a new styled wrapper
    pub fn new(
        content: Element<'a, Message, ThemeType, RendererType>,
        base_style: StyleProperties,
    ) -> Self {
        Self {
            content,
            base_style,
            state_styles: Vec::new(),
            current_state: None,
        }
    }

    /// Add a state-specific style
    pub fn with_state(mut self, state: WidgetState, style: StyleProperties) -> Self {
        self.state_styles.push((state, style));
        self
    }

    /// Set the current state
    pub fn set_state(&mut self, state: Option<WidgetState>) {
        self.current_state = state;
    }

    /// Get the effective style for current state
    pub fn effective_style(&self) -> StyleProperties {
        let mut style = self.base_style.clone();

        // Apply state styles if state is set
        if let Some(state) = self.current_state {
            for (s, state_style) in &self.state_styles {
                if s == &state {
                    // Merge state style over base
                    if let Some(bg) = &state_style.background {
                        style.background = Some(bg.clone());
                    }
                    if let Some(color) = &state_style.color {
                        style.color = Some(*color);
                    }
                    if let Some(border) = &state_style.border {
                        style.border = Some(border.clone());
                    }
                    if let Some(shadow) = &state_style.shadow {
                        style.shadow = Some(*shadow);
                    }
                    if let Some(opacity) = state_style.opacity {
                        style.opacity = Some(opacity);
                    }
                    if let Some(transform) = &state_style.transform {
                        style.transform = Some(transform.clone());
                    }
                }
            }
        }

        style
    }
}

impl<'a, Message, ThemeType, RendererType> iced::Widget<Message, ThemeType, RendererType>
    for Styled<'a, Message, ThemeType, RendererType>
where
    ThemeType: iced::Theme,
    RendererType: iced::Renderer,
{
    fn state(&self) -> iced::widget::State {
        iced::widget::State::new(())
    }

    fn children(&self) -> Vec<iced::widget::State> {
        vec![self.content.state()]
    }

    fn diff(&self, tree: &mut iced::widget::Tree) {
        tree.diff_children(&[&self.content])
    }

    fn width(&self) -> iced::Length {
        self.content.width()
    }

    fn height(&self) -> iced::Length {
        self.content.height()
    }

    fn layout(
        &self,
        tree: &mut iced::widget::Tree,
        renderer: &RendererType,
        limits: &iced::layout::Limits,
    ) -> iced::layout::Node {
        self.content.layout(tree, renderer, limits)
    }

    fn draw(
        &self,
        tree: &iced::widget::Tree,
        renderer: &mut RendererType,
        theme: &ThemeType,
        style: &iced::renderer::Style,
        layout: iced::layout::Geometry<'_>,
        cursor_position: iced::Cursor,
        viewport: &iced::Rectangle,
    ) {
        self.content.draw(
            tree,
            renderer,
            theme,
            style,
            layout,
            cursor_position,
            viewport,
        )
    }

    fn hash_layout(&self, state: &mut iced::Hasher) {
        self.content.hash_layout(state)
    }
}

impl<'a, Message, ThemeType, RendererType> From<Styled<'a, Message, ThemeType, RendererType>>
    for Element<'a, Message, ThemeType, RendererType>
where
    ThemeType: iced::Theme,
    RendererType: iced::Renderer,
    Message: 'a,
{
    fn from(styled: Styled<'a, Message, ThemeType, RendererType>) -> Self {
        Element::new(styled)
    }
}

/// Helper function to create a styled container
pub fn styled_container<'a, Message, ThemeType, RendererType>(
    content: Element<'a, Message, ThemeType, RendererType>,
    style: StyleProperties,
) -> Container<'a, Message, ThemeType, RendererType>
where
    ThemeType: iced::Theme,
    RendererType: iced::Renderer,
{
    use crate::style_mapping::map_style_properties;

    let container_style = map_style_properties(&style);

    container(content).style(move |_theme| container_style)
}
