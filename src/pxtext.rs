use std::ops::Range;

use bevy::prelude::*;

use crate::pxfont::PxFont;

/// This mirrors the `SpriteBundle`, adding text in addition.
#[derive(Debug, Bundle, Clone, Default)]
pub struct PxTextBundle {
    pub text: PxText,
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

/// A section of formatted text
#[derive(Debug, Clone, Default)]
pub struct PxTextSection {
    pub value: String,
    pub color: Color,
    pub underline: bool,
}

impl PxTextSection {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            color: Color::WHITE,
            underline: false,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn underlined(mut self) -> Self {
        self.underline = true;
        self
    }
}

/// Wrap lines of text
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum WrapMode {
    /// Maintain word wholeness
    #[default]
    WrapWord,
    /// Break off words in the middle
    WrapChar,
    /// Don't draw characters out of bounds
    Truncate,
}

#[derive(Debug, Component, Clone, Default)]
pub struct PxText {
    pub sections: Vec<PxTextSection>,
    pub font: Handle<PxFont>,
    pub line_spacing: u32,
    pub wrap_mode: WrapMode,
    pub bounding_box: Option<UVec2>,
}

impl PxText {
    pub fn from_section(value: impl Into<String>, font: Handle<PxFont>) -> Self {
        Self {
            sections: vec![PxTextSection::new(value)],
            font,
            line_spacing: 1,
            wrap_mode: WrapMode::default(),
            bounding_box: None,
        }
    }

    pub fn from_sections(sections: Vec<PxTextSection>, font: Handle<PxFont>) -> Self {
        Self {
            sections,
            font,
            line_spacing: 1,
            wrap_mode: WrapMode::default(),
            bounding_box: None,
        }
    }

    pub fn with_line_spacing(mut self, spacing: u32) -> Self {
        self.line_spacing = spacing;
        self
    }

    pub fn with_truncating_wrap(mut self) -> Self {
        self.wrap_mode = WrapMode::Truncate;
        self
    }

    pub fn with_word_wrap(mut self) -> Self {
        self.wrap_mode = WrapMode::WrapWord;
        self
    }

    pub fn with_char_wrap(mut self) -> Self {
        self.wrap_mode = WrapMode::WrapChar;
        self
    }

    pub fn with_bounding_box(mut self, bounds: UVec2) -> Self {
        self.bounding_box = Some(bounds);
        self
    }
}

/// Pixel text that can be clicked and hovered on.
#[derive(Debug, Component)]
pub enum PickableText {
    /// Sense pick events for a range of characters
    Chars(Range<usize>),
    /// Sense pick events for a range of sections
    Sections(Range<usize>),
    /// Sense pick events for the whole text
    Whole,
}

impl PickableText {
    pub fn get_string(&self, text: &PxText) -> (String, Range<usize>) {
        match self {
            PickableText::Chars(range) => {
                let mut string = String::new();
                let mut index = 0;
                let mut min = None;
                for section in &text.sections {
                    for c in section.value.chars() {
                        if index >= range.start && index < range.end {
                            if min.is_none() {
                                min = Some(index);
                            }

                            string.push(c);
                        } else if index >= range.end {
                            break;
                        }

                        index += 1;
                    }
                }

                if min.is_none() {
                    error!("Pickable r is out of range!");
                }

                (string, min.unwrap()..index)
            }
            PickableText::Sections(range) => {
                let mut string = String::new();
                let mut index = 0;
                let mut min = None;
                for (i, section) in text.sections.iter().enumerate() {
                    if i >= range.start && i < range.end {
                        if min.is_none() {
                            min = Some(index);
                        }

                        string += &section.value;
                    } else if i >= range.end {
                        break;
                    }

                    index += section.value.chars().count();
                }

                if min.is_none() {
                    error!("Pickable r is out of range!");
                }

                (string, min.unwrap()..index)
            }
            PickableText::Whole => {
                let string = text.sections
                    .iter()
                    .flat_map(|s| s.value.chars())
                    .collect();
                let len = text.sections
                    .iter()
                    .map(|s| s.value.len())
                    .reduce(|x, y| x + y)
                    .unwrap_or_default();

                (string, 0..len)
            }
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub(crate) struct PickRect(pub Vec<IRect>);

/// A `PickableText` had a mouse interaction
#[derive(Event, Debug)]
pub struct PxTextEvent {
    /// The entity with the `PxText` component
    pub entity: Entity,
    pub range: Range<usize>,
    pub value: String,
    pub rect: IRect,
    pub pick_type: EventType,
}

impl PxTextEvent {
    pub fn clicked(&self) -> bool {
        self.pick_type == EventType::LeftClick || self.pick_type == EventType::RightClick
    }

    pub fn left_clicked(&self) -> bool {
        self.pick_type == EventType::LeftClick
    }

    pub fn right_clicked(&self) -> bool {
        self.pick_type == EventType::RightClick
    }

    pub fn hovered(&self) -> bool {
        self.pick_type == EventType::Hover
    }
}

#[derive(Debug, PartialEq, Eq)]
/// Type of mouse interaction
pub enum EventType {
    LeftClick,
    RightClick,
    Hover,
}
