use background::Background;
use ril::Rgba;
use sort::Sort;

use crate::alignment::Alignment;
mod background;
mod sort;
pub enum Style {
    // Visibility
    Visible,
    Hidden,

    /// Alignment
    Alignment(Alignment),
    Soring(Sort),

    /// Border
    /// Make frame around objects/object
    Border(usize),

    // thickness
    // Work on:
    // `[Border]`, `[Text]`
    Bold(usize),
    Thin(usize),

    /// Colors
    Color(Rgba),
    Background(Background),

    /// Indents
    /// Between objects
    Gap(usize),
    /// Outside indentation
    Indent(isize),
}

impl Default for Style {
    fn default() -> Self {
        Style::Visible
    }
}

impl Styling for Style {}

pub trait Styling {}
