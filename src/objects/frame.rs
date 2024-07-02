use crate::{objects::Object, size::Size, style::Style};

use super::Objectionable;

pub struct Frame {
    size: Size,
    objects: Vec<Objectionable>,
    style: Vec<Style>,
}
// impl Object for Frame {}
