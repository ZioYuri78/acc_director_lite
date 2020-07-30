use std::fmt;
use std::io::Cursor;
use std::time::Duration;

use byteorder::{NativeEndian, ReadBytesExt};

use crate::accd_lap_info::ACCDLapInfo;
use crate::accd_utils::read_string;

#[derive(Debug, Clone)]
pub enum SessionPhase {
    NONE = 0,
    Starting = 1,
    PreFormation = 2,
    FormationLap = 3,
    PreSession = 4,
    Session = 5,
    SessionOver = 6,
    PostSession = 7,
    ResultUI = 8,
    Error,
}

impl From<u8> for SessionPhase {
    fn from(value: u8) -> Self {
        match value {
            0 => SessionPhase::NONE,
            1 => SessionPhase::Starting,
            2 => SessionPhase::PreFormation,
            3 => SessionPhase::FormationLap,
            4 => SessionPhase::PreSession,
            5 => SessionPhase::Session,
            6 => SessionPhase::SessionOver,
            7 => SessionPhase::PostSession,
            8 => SessionPhase::ResultUI,
            _ => SessionPhase::Error,
        }
    }
}

#[derive(Debug, Clone)]
pub enum RaceSessionType {
    Practice = 0,
    Qualifying = 4,
    Superpole = 9,
    Race = 10,
    Hotlap = 11,
    Hotstint = 12,
    HotlapSuperpole = 13,
    Replay = 14,
    Error,
}

impl From<u8> for RaceSessionType {
    fn from(value: u8) -> Self {
        match value {
            0 => RaceSessionType::Practice,
            4 => RaceSessionType::Qualifying,
            9 => RaceSessionType::Superpole,
            10 => RaceSessionType::Race,
            11 => RaceSessionType::Hotlap,
            12 => RaceSessionType::Hotstint,
            13 => RaceSessionType::HotlapSuperpole,
            14 => RaceSessionType::Replay,
            _ => RaceSessionType::Error,
        }
    }
}

#[allow(unused_variables)]
#[derive(Debug, Clone)]
pub struct ACCDRealtimeUpdate {
    event_index: i32,
    session_index: i32,
    pub phase: SessionPhase,
    pub session_time: Duration,
    pub remaining_time: Duration,
    pub time_of_day: Duration,
    pub rain_level: f32,
    pub clouds: f32,
    pub wetness: f32,
    pub best_session_lap: ACCDLapInfo,
    pub bestlap_car_index: u16,
    pub bestlap_driver_index: u16,
    pub focused_car_index: i32,
    pub active_camera_set: String,
    pub active_camera: String,
    pub is_replay_playing: bool,
    pub replay_session_time: f32,
    pub replay_remaining_time: f32,
    pub session_remaining_time: Duration,
    pub session_end_time: Duration,
    pub session_type: RaceSessionType,
    pub ambient_temp: u8,
    pub track_temp: u8,
    pub current_hud_page: String,
}

impl ACCDRealtimeUpdate {
    pub fn new(cur: &mut Cursor<&Vec<u8>>) -> Self {
        let event_index = cur.read_u16::<NativeEndian>().unwrap() as i32;
        let session_index = cur.read_u16::<NativeEndian>().unwrap() as i32;
        let session_type = RaceSessionType::from(cur.read_u8().unwrap());
        let phase = SessionPhase::from(cur.read_u8().unwrap());
        let session_time = Duration::from_millis(cur.read_f32::<NativeEndian>().unwrap() as u64);
        let session_end_time =
            Duration::from_millis(cur.read_f32::<NativeEndian>().unwrap() as u64);

        let focused_car_index = cur.read_i32::<NativeEndian>().unwrap();
        let active_camera_set = read_string(cur);
        let active_camera = read_string(cur);
        let current_hud_page = read_string(cur);

        let mut replay_session_time: f32 = 0.0;
        let mut replay_remaining_time: f32 = 0.0;
        let is_replay_playing = if cur.read_u8().unwrap() > 0 {
            replay_session_time = cur.read_f32::<NativeEndian>().unwrap();
            replay_remaining_time = cur.read_f32::<NativeEndian>().unwrap();
            true
        } else {
            false
        };

        let time_of_day = Duration::from_millis(cur.read_f32::<NativeEndian>().unwrap() as u64);
        let ambient_temp = cur.read_u8().unwrap();
        let track_temp = cur.read_u8().unwrap();
        let clouds = (cur.read_u8().unwrap() as f32) / 10f32;
        let rain_level = (cur.read_u8().unwrap() as f32) / 10f32;
        let wetness = (cur.read_u8().unwrap() as f32) / 10f32;

        let best_session_lap = ACCDLapInfo::new(cur);

        ACCDRealtimeUpdate {
            event_index,
            session_index,
            phase,
            session_time,
            remaining_time: Duration::from_millis(0),
            time_of_day,
            rain_level,
            clouds,
            wetness,
            best_session_lap,
            bestlap_car_index: 0,
            bestlap_driver_index: 0,
            focused_car_index,
            active_camera_set,
            active_camera,
            is_replay_playing,
            replay_session_time,
            replay_remaining_time,
            session_remaining_time: Duration::from_millis(0),
            session_end_time,
            session_type,
            ambient_temp,
            track_temp,
            current_hud_page,
        }
    }
}

impl fmt::Display for ACCDRealtimeUpdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "/== Realtime Update ==/")?;
        writeln!(
            f,
            "event id: {} | session id: {} | phase: {:#?}",
            self.event_index, self.session_index, self.phase
        )?;
        writeln!(
            f,
            "session type: {:#?} | end time: {:?} | remaining time(?): {:?}",
            self.session_type, self.session_end_time, self.remaining_time
        )?;
        writeln!(
            f,
            "session time: {:?} | remaining time: {:?}",
            self.session_time, self.session_remaining_time
        )?;
        writeln!(
            f,
            "time of day: {:?} | ambient temp: {} | track temp: {}",
            self.time_of_day, self.ambient_temp, self.track_temp
        )?;
        writeln!(
            f,
            "clouds: {} | rain level: {} | wetness: {}",
            self.clouds, self.rain_level, self.wetness
        )?;
        writeln!(
            f,
            "car id: {} | driver id: {} | best lap:\n {}",
            self.bestlap_car_index, self.bestlap_driver_index, self.best_session_lap
        )?;
        writeln!(
            f,
            "{} | {} | focused car: {}",
            self.active_camera_set, self.active_camera, self.focused_car_index
        )?;
        writeln!(
            f,
            "replay playing: {} | session time: {} | remaining time: {}",
            self.is_replay_playing, self.replay_session_time, self.replay_remaining_time
        )?;
        writeln!(f, "current hud: {}", self.current_hud_page)?;
        writeln!(f, "/---------------------/")
    }
}

impl Default for ACCDRealtimeUpdate {
    fn default() -> Self {
        ACCDRealtimeUpdate {
            event_index: -1,
            session_index: -1,
            phase: SessionPhase::NONE,
            session_time: Duration::from_secs(0),
            remaining_time: Duration::from_secs(00),
            time_of_day: Duration::from_secs(0),
            rain_level: -1.0,
            clouds: -1.0,
            wetness: -1.0,
            best_session_lap: ACCDLapInfo::default(),
            bestlap_car_index: 65535,
            bestlap_driver_index: 65535,
            focused_car_index: -1,
            active_camera_set: "".to_string(),
            active_camera: "".to_string(),
            is_replay_playing: false,
            replay_session_time: -1.0,
            replay_remaining_time: -1.0,
            session_remaining_time: Duration::from_secs(0),
            session_end_time: Duration::from_secs(0),
            session_type: RaceSessionType::Error,
            ambient_temp: 255,
            track_temp: 255,
            current_hud_page: "".to_string(),
        }
    }
}
