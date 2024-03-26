#[derive(Default)]
pub struct UiWidjetsInfo {
    widget_id_count: u32,
    pub widget_selected: u32,
}

impl UiWidjetsInfo {
    pub fn new_widget_id(&mut self) -> u32 {
        let out = self.widget_id_count;
        self.widget_id_count += 1;
        return out;
    }
}
