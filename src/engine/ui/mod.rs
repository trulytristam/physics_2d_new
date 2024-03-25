pub mod widjets;
use std::rc::Rc;

use widjets::Widjet;

use self::widjets::ui_widjet_info;

#[derive(Default)]
pub struct Ui {
    widjets_info: ui_widjet_info::UiWidjetsInfo,
    widjets: Vec<Rc<dyn Widjet>>,
}

impl Ui {
    fn update(&mut self) {}

    fn delete_flagged(&mut self) {
        self.widjets.retain(|e| !e.get_delete());
    }
}
