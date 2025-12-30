/// Backend for rendering IR to a specific UI framework
pub trait Backend {
    /// The widget type produced by this backend
    type Widget<'a>;

    /// The message type for events
    type Message: Clone + 'static;

    /// Create a text widget
    fn text<'a>(&self, content: &str) -> Self::Widget<'a>;

    /// Create a button widget
    fn button<'a>(
        &self,
        label: Self::Widget<'a>,
        on_press: Option<Self::Message>,
    ) -> Self::Widget<'a>;

    /// Create a column layout
    fn column<'a>(&self, children: Vec<Self::Widget<'a>>) -> Self::Widget<'a>;

    /// Create a row layout
    fn row<'a>(&self, children: Vec<Self::Widget<'a>>) -> Self::Widget<'a>;
}
