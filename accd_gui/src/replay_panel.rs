extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgPartial;

use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use accd2::accd_protocol::ACCDProtocol;
use accd2::accd_realtime_update::ACCDRealtimeUpdate;

#[derive(Default, NwgPartial)]
pub struct ReplayPanel {
    #[nwg_control(text: "Replay")]
    replay_tab: nwg::Tab,

    #[nwg_layout(parent: replay_tab, spacing: 1)]
    replay_grid: nwg::GridLayout,

    #[nwg_control(parent: replay_tab, text: "Replay last 10s")]
    #[nwg_layout_item(layout: replay_grid, row: 0, col: 0)]
    pub replay_10s_btn: nwg::Button,

    #[nwg_control(parent: replay_tab, text: "Replay last 30s")]
    #[nwg_layout_item(layout: replay_grid, row: 0, col: 1)]
    pub replay_30s_btn: nwg::Button,
}
