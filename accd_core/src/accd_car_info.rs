use std::fmt;

use crate::accd_driver_info::ACCDDriverInfo;
use crate::accd_enums::NationalityEnum;

#[derive(Debug, Clone, PartialEq)]
pub struct ACCDCarInfo {
    pub car_index: u16,
    pub car_model_type: u8,
    pub team_name: String,
    pub race_number: i32,
    pub cup_category: u8,
    pub current_driver_index: i32,
    pub drivers: Vec<ACCDDriverInfo>,
    pub nationality: NationalityEnum,
}

impl ACCDCarInfo {
    pub fn new(car_index: u16) -> Self {
        ACCDCarInfo {
            car_index,
            ..ACCDCarInfo::default()
        }
    }
}

impl Default for ACCDCarInfo {
    fn default() -> Self {
        ACCDCarInfo {
            car_index: 65535,
            car_model_type: 255,
            team_name: "".into(),
            race_number: -1,
            cup_category: 255,
            current_driver_index: -1,
            drivers: Vec::new(),
            nationality: NationalityEnum::Error,
        }
    }
}

impl fmt::Display for ACCDCarInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "/== Car Info ==/\r\n")?;
        writeln!(
            f,
            "id: {} | car: {} | #: {}\r\n",
            self.car_index, self.car_model_type, self.race_number
        )?;
        writeln!(f, "{} ({:#?})", self.team_name, self.nationality)?;
        writeln!(
            f,
            "cup cat: {} | driver id:{}\r\n",
            self.cup_category, self.current_driver_index
        )?;
        writeln!(f, "{:#?}\r\n", self.drivers)?;

        writeln!(f, "/---------------/\r\n")
    }
}
