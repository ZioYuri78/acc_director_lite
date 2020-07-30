extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgPartial;

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default, NwgPartial)]
pub struct CameraPanel {
    #[nwg_control(text: "Cameras")]
    pub camera_tab: nwg::Tab,

    #[nwg_layout(parent: camera_tab, spacing: 1)]
    pub camera_grid: nwg::GridLayout,

    #[nwg_control]
    pub camera_notice: nwg::Notice,

    pub camera_data: Arc<Mutex<HashMap<String, Vec<String>>>>,
    pub camera_buttons: RefCell<Vec<nwg::Button>>,
    pub camera_handlers: RefCell<Vec<nwg::EventHandler>>,
}
