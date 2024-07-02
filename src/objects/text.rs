use crate::style::{Style, Styling};

use super::Object;

pub struct Text {
    text: String,
    style: Vec<Style>,
}

// impl Object for Text {}
impl Styling for Text {}
