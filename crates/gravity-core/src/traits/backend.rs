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

    /// Create a container widget
    fn container<'a>(&self, content: Self::Widget<'a>) -> Self::Widget<'a>;

    /// Create a scrollable widget
    fn scrollable<'a>(&self, content: Self::Widget<'a>) -> Self::Widget<'a>;

    /// Create a stack widget
    fn stack<'a>(&self, children: Vec<Self::Widget<'a>>) -> Self::Widget<'a>;

    /// Create a text input widget
    fn text_input<'a>(
        &self,
        placeholder: &str,
        value: &str,
        on_input: Option<Self::Message>,
    ) -> Self::Widget<'a>;

    /// Create a checkbox widget
    fn checkbox<'a>(
        &self,
        label: &str,
        is_checked: bool,
        on_toggle: Option<Self::Message>,
    ) -> Self::Widget<'a>;

    /// Create a slider widget
    fn slider<'a>(
        &self,
        min: f32,
        max: f32,
        value: f32,
        on_change: Option<Self::Message>,
    ) -> Self::Widget<'a>;

    /// Create a pick list widget
    fn pick_list<'a>(
        &self,
        options: Vec<&str>,
        selected: Option<&str>,
        on_select: Option<Self::Message>,
    ) -> Self::Widget<'a>;

    /// Create a toggler widget
    fn toggler<'a>(
        &self,
        label: &str,
        is_active: bool,
        on_toggle: Option<Self::Message>,
    ) -> Self::Widget<'a>;

    /// Create an image widget
    fn image<'a>(&self, path: &str) -> Self::Widget<'a>;

    /// Create an SVG widget
    fn svg<'a>(&self, path: &str) -> Self::Widget<'a>;

    /// Create a space widget (flexible spacing)
    fn space<'a>(&self) -> Self::Widget<'a>;

    /// Create a rule widget (divider)
    fn rule<'a>(&self) -> Self::Widget<'a>;
}
