pub enum Alignment {
    // all top
    Top(Top),
    // all bottom
    Bottom(Bottom),
    // all center
    Center(Center),
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Center(Center::Center)
    }
}

impl Alignment {
    pub fn directional(&self, size: (u32, u32)) -> (u32, u32) {
        let pie = (size.0 / 3, size.1 / 3);
        let piee = (size.0 / 3 * 2, size.1 / 3 * 2);
        match self {
            Alignment::Top(e) => match e {
                Top::Top => (1, pie.1 + 1),
                Top::LeftTop => (1, 1),
                Top::RightTop => (1, piee.1 + 1),
            },
            Alignment::Bottom(e) => match e {
                Bottom::Bottom => (1, piee.1 + 1),
                Bottom::LeftBottom => (1, piee.1 + 1),
                Bottom::RightBottom => (1, pie.1 + 1),
            },
            Alignment::Center(e) => match e {
                Center::Center => (1 + pie.0, pie.1 + 1),
                Center::LeftCenter => (1, pie.1 + 1),
                Center::RightCenter => (1 + piee.0, pie.1 + 1),
            },
        }
    }
}

#[derive(Default)]
pub enum Left {
    #[default]
    Left,
    LeftTop,
    LeftBottom,
}

#[derive(Default)]
pub enum Top {
    #[default]
    Top,
    LeftTop,
    RightTop,
}

#[derive(Default)]
pub enum Right {
    #[default]
    Right,
    RightTop,
    RightBottom,
}

#[derive(Default)]
pub enum Bottom {
    #[default]
    Bottom,
    LeftBottom,
    RightBottom,
}

#[derive(Default)]
pub enum Center {
    #[default]
    Center,
    LeftCenter,
    RightCenter,
}
