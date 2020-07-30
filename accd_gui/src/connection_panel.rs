extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgPartial;

use std::sync::{Arc, Mutex};

use accd2::accd_registration_result::ACCDRegistrationResult;

#[derive(Default, NwgPartial)]
pub struct ConnectionPanel {
    #[nwg_control(text: "Connection")]
    connection_tab: nwg::Tab,

    #[nwg_layout(parent: connection_tab, spacing: 1)]
    connection_grid: nwg::GridLayout,

    #[nwg_control(parent: connection_tab, readonly: true, text: "Panel that manage connection\r\n settings to the game.")]
    #[nwg_layout_item(layout: connection_grid, row: 0, col: 0)]
    pub reg_result_tb: nwg::TextBox,

    #[nwg_control]    
    #[nwg_events(OnNotice:[ConnectionPanel::update_registration_tab])]
    pub registration_notice: nwg::Notice,

    pub registration_data: Arc<Mutex<ACCDRegistrationResult>>,
}

impl ConnectionPanel {
    fn update_registration_tab(&self) {
        let reg_result = self.registration_data.lock().unwrap();
        self.reg_result_tb.set_text(&format!(
            "Id: {}\r\n\
            Connected: {}\r\n\
            Read only: {}\r\n\
            Msg: {}",
            reg_result.connection_id,
            reg_result.connection_success,
            reg_result.is_read_only,
            reg_result.err_msg
        ));
    }
}