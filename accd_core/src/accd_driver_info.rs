use std::fmt;
use std::io::Cursor;

use crate::accd_enums::NationalityEnum;
use crate::accd_utils::read_string;

use byteorder::*;

#[derive(Debug, Clone)]
pub enum DriverCategory {
    Platinum = 3,
    Gold = 2,
    Silver = 1,
    Bronze = 0,
    Error = 255,
}

impl From<u8> for DriverCategory {
    fn from(value: u8) -> Self {
        match value {
            0 => DriverCategory::Bronze,
            1 => DriverCategory::Silver,
            2 => DriverCategory::Gold,
            3 => DriverCategory::Platinum,
            _ => DriverCategory::Error,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ACCDDriverInfo {
    pub first_name: String,
    pub last_name: String,
    pub short_name: String,
    pub category: DriverCategory,
    pub nationality: NationalityEnum,
}

impl ACCDDriverInfo {
    pub fn new(cur: &mut Cursor<&Vec<u8>>) -> Self {
        ACCDDriverInfo {
            first_name: read_string(cur),
            last_name: read_string(cur),
            short_name: read_string(cur),
            category: DriverCategory::from(cur.read_u8().unwrap()),
            nationality: NationalityEnum::from(cur.read_u16::<NativeEndian>().unwrap() as u8),
        }
    }
}

impl fmt::Display for ACCDDriverInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "/== Driver Info ==/\r\n")?;
        writeln!(
            f,
            "{} {} ({:#?})\r\n",
            self.first_name, self.last_name, self.nationality
        )?;
        writeln!(f, "{}\r\n", self.short_name)?;
        writeln!(f, "{:#?}\r\n", self.category)?;
        writeln!(f, "/-----------------/\r\n")
    }
}

impl Default for ACCDDriverInfo {
    fn default() -> Self {
        ACCDDriverInfo {
            first_name: "".to_string(),
            last_name: "".to_string(),
            short_name: "".to_string(),
            category: DriverCategory::Error,
            nationality: NationalityEnum::Error,
        }
    }
}
