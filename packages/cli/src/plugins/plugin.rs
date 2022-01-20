use crate::data;

use super::PluginResult;

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn run(&self, data: &data::Data) -> PluginResult;
}
