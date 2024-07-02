pub mod frame;
pub mod text;

enum Objectionable
where
    Self: Object,
{
    Text(text::Text),
    Frame(frame::Frame),
}

impl Object for Objectionable {
    fn pie(&self) -> (u32, u32) {
        todo!()
    }
}
pub trait Object {
    /// Draw object
    fn eat(&self) {}
    /// 1/3 area of object
    fn pie(&self) -> (u32, u32);
}
