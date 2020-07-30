extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgPartial;

use std::sync::{Arc, Mutex};

use accd2::accd_realtime_update::ACCDRealtimeUpdate;

#[derive(Default, NwgPartial)]
pub struct RealtimeUpdatePanel {
    #[nwg_control(text: "Realtime Update")]
    update_tab: nwg::Tab,

    #[nwg_layout(parent: update_tab, spacing: 1)]
    update_grid: nwg::GridLayout,

    #[nwg_control(parent: update_tab, readonly: true, text: "Panel with realtime updates")]
    #[nwg_layout_item(layout: update_grid, row: 0, col: 0)]
    pub realtime_tb: nwg::TextBox,

    #[nwg_control]
    #[nwg_events(OnNotice:[RealtimeUpdatePanel::update_realtime_tab])]
    pub realtime_update_notice: nwg::Notice,

    pub realtime_update_data: Arc<Mutex<ACCDRealtimeUpdate>>,
}

impl RealtimeUpdatePanel {
    fn update_realtime_tab(&self) {
        let realtime_data = self.realtime_update_data.lock().unwrap();

        self.realtime_tb.clear();
        self.realtime_tb.set_text(&format!(
            "Ambient Temp: {}\r\n\
            Track Temp: {}\r\n\
            Time of day: {:?}\r\n\
            Phase: {:?}\r\n\
            Hud Page: {}\r\n\
            Camera: {}",
            realtime_data.ambient_temp,
            realtime_data.track_temp,
            realtime_data.time_of_day,
            realtime_data.phase,
            realtime_data.current_hud_page,
            realtime_data.active_camera
        ));
    }
}
