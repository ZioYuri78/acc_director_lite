use std::fmt;
use std::io::Cursor;

use byteorder::{NativeEndian, ReadBytesExt};

use crate::accd_car_info::ACCDCarInfo;
use crate::accd_protocol::ACCDProtocol;
use crate::accd_utils::read_string;

#[derive(Debug, Clone)]
pub enum BroadcastingCarEventType {
    None = 0,
    GreenFlag = 1,
    SessionOver = 2,
    PenaltyCommMsg = 3,
    Accident = 4,
    LapCompleted = 5,
    BestSessionLap = 6,
    BestPersonalLap = 7,
    Error,
}

impl From<u8> for BroadcastingCarEventType {
    fn from(value: u8) -> Self {
        match value {
            0 => BroadcastingCarEventType::None,
            1 => BroadcastingCarEventType::GreenFlag,
            2 => BroadcastingCarEventType::SessionOver,
            3 => BroadcastingCarEventType::PenaltyCommMsg,
            4 => BroadcastingCarEventType::Accident,
            5 => BroadcastingCarEventType::LapCompleted,
            6 => BroadcastingCarEventType::BestSessionLap,
            7 => BroadcastingCarEventType::BestPersonalLap,
            _ => BroadcastingCarEventType::Error,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ACCDBroadcastingEvent {
    pub event_type: BroadcastingCarEventType,
    pub event_msg: String,
    pub event_time_ms: i32,
    pub event_car_id: i32,
    pub event_car_data: ACCDCarInfo,
}

impl ACCDBroadcastingEvent {
    pub fn new(cur: &mut Cursor<&Vec<u8>>, accd_conn: &ACCDProtocol) -> Self {
        let event_type = BroadcastingCarEventType::from(cur.read_u8().unwrap());
        let event_msg = read_string(cur);
        let event_time_ms = cur.read_i32::<NativeEndian>().unwrap();
        let event_car_id = cur.read_i32::<NativeEndian>().unwrap();
        let event_car_data = match accd_conn
            .entry_list_cars
            .iter()
            .find(|car_info| car_info.car_index == event_car_id as u16)
        {
            Some(car_info) => car_info.clone(),

            None => ACCDCarInfo::default().clone(),
        };

        ACCDBroadcastingEvent {
            event_type,
            event_msg,
            event_time_ms,
            event_car_id,
            event_car_data,
        }
    }
}

impl Default for ACCDBroadcastingEvent {
    fn default() -> Self {
        ACCDBroadcastingEvent {
            event_type: BroadcastingCarEventType::Error,
            event_msg: "".to_string(),
            event_time_ms: -1,
            event_car_id: -1,
            event_car_data: ACCDCarInfo::default(),
        }
    }
}

impl fmt::Display for ACCDBroadcastingEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "/== Broadcasting Event ==/")?;
        writeln!(f, "{:#?}", self.event_type)?;
        writeln!(f, "{}", self.event_msg)?;
        writeln!(f, "{}", self.event_time_ms)?;
        writeln!(f, "\n{}", self.event_car_data)?;
        writeln!(f, "/------------------------/")
    }
}
