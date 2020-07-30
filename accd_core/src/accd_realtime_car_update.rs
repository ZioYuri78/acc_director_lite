use std::fmt;
use std::io::Cursor;

use byteorder::{NativeEndian, ReadBytesExt};

use crate::accd_lap_info::ACCDLapInfo;

#[derive(Debug, Clone)]
pub enum CarLocationEnum {
    NONE = 0,
    Track = 1,
    Pitlane = 2,
    PitEntry = 3,
    PitExit = 4,
    Error,
}

impl From<u8> for CarLocationEnum {
    fn from(value: u8) -> Self {
        match value {
            0 => CarLocationEnum::NONE,
            1 => CarLocationEnum::Track,
            2 => CarLocationEnum::Pitlane,
            3 => CarLocationEnum::PitEntry,
            4 => CarLocationEnum::PitExit,
            _ => CarLocationEnum::Error,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ACCDRealtimeCarUpdate {
    pub car_index: i32,
    pub driver_index: i32,
    pub gear: i32,
    pub world_pos_x: f32,
    pub world_pos_y: f32,
    pub yaw: f32,
    pub car_location: CarLocationEnum,
    pub kmh: i32,
    pub position: i32,
    pub track_position: i32,
    pub spline_position: f32,
    pub delta: i32,
    pub best_session_lap: ACCDLapInfo,
    pub last_lap: ACCDLapInfo,
    pub current_lap: ACCDLapInfo,
    pub laps: i32,
    pub cup_position: u16,
    pub driver_count: u8,
}

impl ACCDRealtimeCarUpdate {
    pub fn new(
        cur: &mut Cursor<&Vec<u8>>,
        car_index: i32,
        driver_index: i32,
        driver_count: u8,
    ) -> Self {
        ACCDRealtimeCarUpdate {
            car_index,
            driver_index,
            driver_count,
            gear: (cur.read_u8().unwrap() as i32 - 1) as i32,
            world_pos_x: cur.read_f32::<NativeEndian>().unwrap(),
            world_pos_y: cur.read_f32::<NativeEndian>().unwrap(),
            yaw: cur.read_f32::<NativeEndian>().unwrap(),
            car_location: CarLocationEnum::from(cur.read_u8().unwrap()),
            kmh: cur.read_u16::<NativeEndian>().unwrap() as i32,
            position: cur.read_u16::<NativeEndian>().unwrap() as i32,
            cup_position: cur.read_u16::<NativeEndian>().unwrap(),
            track_position: cur.read_u16::<NativeEndian>().unwrap() as i32,
            spline_position: cur.read_f32::<NativeEndian>().unwrap(),
            laps: cur.read_u16::<NativeEndian>().unwrap() as i32,
            delta: cur.read_i32::<NativeEndian>().unwrap(),
            best_session_lap: ACCDLapInfo::new(cur),
            last_lap: ACCDLapInfo::new(cur),
            current_lap: ACCDLapInfo::new(cur),
        }
    }
}

impl fmt::Display for ACCDRealtimeCarUpdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "/== Realtime Car Update ==/\r\n")?;
        writeln!(
            f,
            "id: {} | driver: {} ({})\r\n",
            self.car_index, self.driver_index, self.driver_count
        )?;
        writeln!(f, "location: {:#?}", self.car_location)?;
        writeln!(
            f,
            "X: {} Y: {} | yaw: {}\r\n",
            self.world_pos_x, self.world_pos_y, self.yaw
        )?;
        writeln!(
            f,
            "track pos: {} | spline pos: {}\r\n",
            self.track_position, self.spline_position
        )?;
        writeln!(
            f,
            "pos: {} | cup pos: {}\r\n",
            self.position, self.cup_position
        )?;
        writeln!(f, "kmh: {} | gear: {}\r\n", self.kmh, self.gear)?;
        writeln!(f, "laps: {} | delta: {}\r\n", self.laps, self.delta)?;
        writeln!(f, "best lap:\r\n {}", self.best_session_lap)?;
        writeln!(f, "last lap:\r\n {}", self.last_lap)?;
        writeln!(f, "current lap:\r\n {}", self.current_lap)?;
        writeln!(f, "/-------------------------/")
    }
}

impl Default for ACCDRealtimeCarUpdate {
    fn default() -> Self {
        ACCDRealtimeCarUpdate {
            car_index: -1,
            driver_index: -1,
            gear: -1000,
            world_pos_x: 0.0,
            world_pos_y: 0.0,
            yaw: 0.0,
            car_location: CarLocationEnum::Error,
            kmh: -1,
            position: -1,
            track_position: -1,
            spline_position: 0.0,
            delta: 0,
            best_session_lap: ACCDLapInfo::default(),
            last_lap: ACCDLapInfo::default(),
            current_lap: ACCDLapInfo::default(),
            laps: -1,
            cup_position: 0,
            driver_count: 255,
        }
    }
}
