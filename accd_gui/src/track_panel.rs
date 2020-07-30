extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgPartial;

use std::sync::{Arc, Mutex};

use accd2::accd_track_data::ACCDTrackData;

#[derive(Default, NwgPartial)]
pub struct TrackPanel {
    #[nwg_control(text: "Track")]
    track_tab: nwg::Tab,

    #[nwg_layout(parent: track_tab, spacing: 1)]
    pub track_grid: nwg::GridLayout,

    #[nwg_control(parent: track_tab, readonly: true, text: "Panel that show track informations.")]
    #[nwg_layout_item(layout: track_grid, row: 0, col: 0)]
    pub track_info_tb: nwg::TextBox,

    #[nwg_control]
    #[nwg_events(OnNotice:[TrackPanel::update_track_info])]
    pub track_notice: nwg::Notice,

    pub track_data: Arc<Mutex<ACCDTrackData>>,
}

impl TrackPanel {
    fn update_track_info(&self) {
        let track_data = self.track_data.lock().unwrap().clone();
        self.track_info_tb.set_text(&format!(
            "{}\r\n({}m)",
            track_data.track_name, track_data.track_meters
        )); 
    }
}