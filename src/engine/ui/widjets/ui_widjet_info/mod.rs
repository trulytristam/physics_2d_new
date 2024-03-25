use crate::engine::objects::MP;

use super::Widjet;

#[derive(Default)]
pub struct UiWidjetsInfo {
    widget_id_count: u32,
    widget_selected: Option<MP<dyn Widjet>>,
}
