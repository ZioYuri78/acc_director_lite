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
            ---\r\n\
            Time of day: {:?}\r\n\
            Phase: {:?}\r\n\
            ---\r\n\
            Hud Page: {}\r\n\
            Camera: {}\r\n\
            ---\r\n\
            Rain Level: {}\r\n\
            Clouds: {}\r\n\
            Wetness: {}\r\n\
            ---\r\n\
            Best Session Lap: {}\r\n\
            Best lap car: {}\r\n\
            Best lap driver: {}\r\n\
            ---\r\n\
            Session type: {:?}\r\n\
            Session time: {:?}\r\n\
            Session end time: {:?}\r\n\
            Session remaining time: {:?}\r\n\
            ({:?})\r\n\
            ---\r\n\
            Replay playing {}\r\n\
            Replay sesison time: {}\r\n\
            Replay remaining time: {}",
            realtime_data.ambient_temp,
            realtime_data.track_temp,
            realtime_data.time_of_day,
            realtime_data.phase,
            realtime_data.current_hud_page,
            realtime_data.active_camera,
            realtime_data.rain_level,
            realtime_data.clouds,
            realtime_data.wetness,
            realtime_data.best_session_lap,
            realtime_data.bestlap_car_index,
            realtime_data.bestlap_driver_index,
            realtime_data.session_type,
            realtime_data.session_time,
            realtime_data.session_end_time,
            realtime_data.session_remaining_time,
            realtime_data.remaining_time,
            realtime_data.is_replay_playing,
            realtime_data.replay_session_time,
            realtime_data.replay_remaining_time
        ));
    }
}
