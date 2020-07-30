extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgPartial;

use std::sync::{Arc, Mutex};
use std::cell::RefCell;

use crate::broadcasting_event_data::BroadcastingEventData;
use crate::custom_buttons::BroadcastEvtButton;

#[derive(Default, NwgPartial)]
pub struct EventsPanel {
    #[nwg_control(text: "Events")]
    pub events_tab: nwg::Tab,

    #[nwg_layout(parent: events_tab, spacing: 1)]
    pub events_grid: nwg::GridLayout,

    #[nwg_control]    
    pub broadcasting_event_notice: nwg::Notice,

    pub  broadcasting_event_data: Arc<Mutex<BroadcastingEventData>>,

    pub broadcast_evt_buttons: RefCell<Vec<BroadcastEvtButton>>,
    pub broadcast_evt_handlers: RefCell<Vec<nwg::EventHandler>>,

}