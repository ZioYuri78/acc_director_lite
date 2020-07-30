use accd2::accd_broadcasting_event::ACCDBroadcastingEvent;

#[derive(Clone)]
pub struct BroadcastingEventData {
    pub event: ACCDBroadcastingEvent,
    pub replay_seconds_back: f32,
    pub replay_duration: f32,
}

impl Default for BroadcastingEventData {
    fn default() -> Self {
        BroadcastingEventData {
            event: ACCDBroadcastingEvent::default(),
            replay_seconds_back: 0.0,
            replay_duration: 0.0,
        }
    }
}
