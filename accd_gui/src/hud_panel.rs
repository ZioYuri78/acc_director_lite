extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgPartial;

use std::cell::RefCell;
use std::sync::{Arc, Mutex};

#[derive(Default, NwgPartial)]
pub struct HudPanel {
    #[nwg_control(text: "Huds")]
    pub huds_tab: nwg::Tab,

    #[nwg_layout(parent: huds_tab, spacing: 1)]
    pub huds_grid: nwg::GridLayout,

    #[nwg_control]
    pub huds_notice: nwg::Notice,

    pub huds_data: Arc<Mutex<Vec<String>>>,
    pub hud_buttons: RefCell<Vec<nwg::Button>>,
    pub hud_handlers: RefCell<Vec<nwg::EventHandler>>,
}
