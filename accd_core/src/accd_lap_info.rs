use std::fmt;
use std::io::Cursor;

use byteorder::{NativeEndian, ReadBytesExt};

#[derive(Debug, Clone)]
pub enum LapType {
    Error = 0,
    Outlap = 1,
    Regular = 2,
    Inlap = 3,
}

impl From<u8> for LapType {
    fn from(value: u8) -> Self {
        match value {
            1 => LapType::Outlap,
            2 => LapType::Regular,
            3 => LapType::Inlap,
            _ => LapType::Error,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ACCDLapInfo {
    pub lap_time_ms: i32, //maybe use Option
    pub splits: Vec<i32>, //maybe use Vec<Option>
    pub car_index: u16,
    pub driver_index: u16,
    pub is_invalid: bool,
    pub is_valid_for_best: bool,
    pub lap_type: LapType,
}

impl ACCDLapInfo {
    pub fn new(cur: &mut Cursor<&Vec<u8>>) -> ACCDLapInfo {
        let lap_time_ms = cur.read_i32::<NativeEndian>().unwrap();

        let car_index = cur.read_u16::<NativeEndian>().unwrap();
        let driver_index = cur.read_u16::<NativeEndian>().unwrap();

        let split_count = cur.read_u8().unwrap();
        let mut splits: Vec<i32> = Vec::new();
        for _i in 0..split_count {
            splits.push(cur.read_i32::<NativeEndian>().unwrap());
        }

        let is_invalid = if cur.read_u8().unwrap() > 0 {
            true
        } else {
            false
        };

        let is_valid_for_best = if cur.read_u8().unwrap() > 0 {
            true
        } else {
            false
        };

        let is_out_lap = if cur.read_u8().unwrap() > 0 {
            true
        } else {
            false
        };

        let is_in_lap = if cur.read_u8().unwrap() > 0 {
            true
        } else {
            false
        };

        let lap_type: LapType;
        if is_out_lap {
            lap_type = LapType::Outlap;
        } else if is_in_lap {
            lap_type = LapType::Inlap;
        } else {
            lap_type = LapType::Regular;
        };

        ACCDLapInfo {
            lap_time_ms,
            splits,
            car_index,
            driver_index,
            is_invalid,
            is_valid_for_best,
            lap_type,
        }
    }

    fn to_string(&self) -> String {
        let v = self
            .splits
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let s = self.lap_time_ms.to_string() + &v.join("|");
        s
    }
}

impl Default for ACCDLapInfo {
    fn default() -> Self {
        ACCDLapInfo {
            lap_time_ms: 0,
            splits: Vec::new(),
            car_index: 0,
            driver_index: 0,
            is_invalid: false,
            is_valid_for_best: false,
            lap_type: LapType::Error,
        }
    }
}

impl fmt::Display for ACCDLapInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "/== Lap Info ==/")?;
        writeln!(f, "{:#?}", self.lap_type)?;
        writeln!(
            f,
            "car id:{} | driver id:{}",
            self.car_index, self.driver_index
        )?;
        writeln!(f, "{}", self.to_string())?;
        writeln!(f, "is invalid: {}", self.is_invalid)?;
        writeln!(f, "is valid for best: {}", self.is_valid_for_best)?;
        writeln!(f, "/--------------/")
    }
}
