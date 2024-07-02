use ril::Rgba;

pub enum Background {
    Color(Rgba),
    // Image(),
}

impl Default for Background {
    fn default() -> Self {
        Background::Color(Rgba::default())
    }
}
