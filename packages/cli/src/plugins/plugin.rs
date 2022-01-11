use crate::data;

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn run(&self, data: &data::Data) -> String;
}
