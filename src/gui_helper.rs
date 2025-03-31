// gui_helper.rs

/// Experimenting: I want the code for iced GUI to be more readable
/// Something that is similar to HTML with attributes and elements
/// Writing code with infinite concatenation of methods is fun, but it is not nice to maintain, copy and paste.
/// I would like every line to be as independent as possible.
/// I want to have all attributes in one single place to resemble HTML attributes.

#[derive(Default)]
pub struct XText {}

#[derive(Default)]
pub struct XTextAttr {
    pub size: f32,
}

impl<'a> XText {
    // Rust does not have named parameters and optional parameters. Therefore there must be a bunch of different methods because of the combination of parameters.
    // For readability there cannot be just one function with all the parameters.

    /// a simple text with no attr
    /// the parameter can be String or &str, therefore Cow
    pub fn text(text: impl iced::widget::text::IntoFragment<'a>) -> iced::widget::Text<'a> {
        iced::widget::Text::new(text)
    }

    /// a text with attr
    /// the parameter can be String or &str, therefore Cow
    pub fn attr_text(attr: XTextAttr, text: impl iced::widget::text::IntoFragment<'a>) -> iced::widget::Text<'a> {
        iced::widget::Text::new(text).size(attr.size)
    }
}

pub struct XColumn<'a> {
    attr: XColumnAttr,
    children: Vec<iced::Element<'a, crate::Message>>,
}

pub struct XColumnAttr {
    pub spacing: f32,
    pub padding: iced::Padding,
    pub align_x: iced::alignment::Horizontal,
    pub width: iced::Length,
}

impl Default for XColumnAttr {
    fn default() -> Self {
        XColumnAttr {
            spacing: 0.0,
            padding: iced::Padding::default(),
            align_x: iced::alignment::Horizontal::Center,
            width: iced::Length::Shrink,
        }
    }
}

impl<'a> XColumn<'a> {
    // Rust does not have named parameters and optional parameters. Therefore there must be a bunch of different methods because of the combination of parameters.
    // For readability there cannot be just one function with all the parameters.

    /// a simple text with no attr
    pub fn new(attr: XColumnAttr) -> Self {
        XColumn { attr, children: vec![] }
    }

    // push only one child
    pub fn push(&mut self, child: impl Into<iced::Element<'a, crate::Message>>) {
        self.children.push(child.into());
    }

    // append vec of children
    pub fn append(&mut self, children: &mut Vec<iced::Element<'a, crate::Message>>) {
        self.children.append(children);
    }

    // last possible method, consumes self
    pub fn to_iced(self) -> iced::widget::Column<'a, crate::Message> {
        iced::widget::Column::from_vec(self.children)
            .spacing(self.attr.spacing)
            .padding(self.attr.padding)
            .align_x(self.attr.align_x)
            .width(self.attr.width)
            .into()
    }
}

pub struct XScrollable {}

pub struct XScrollableAttr {
    pub width: iced::Length,
    pub height: iced::Length,
    pub direction: iced::widget::scrollable::Direction,
}

impl Default for XScrollableAttr {
    fn default() -> Self {
        XScrollableAttr {
            width: iced::Length::Shrink,
            height: iced::Length::Shrink,
            direction: iced::widget::scrollable::Direction::default(),
        }
    }
}

impl<'a> XScrollable {
    pub fn new(
        attr: XScrollableAttr,
        content: impl Into<iced::Element<'a, crate::Message>>,
    ) -> iced::widget::Scrollable<'a, crate::Message> {
        iced::widget::scrollable(content)
            .direction(attr.direction)
            .width(attr.width)
            .height(attr.height)
    }
}
