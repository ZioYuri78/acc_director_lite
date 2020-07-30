use std::collections::HashMap;
use std::fmt;
use std::io::Cursor;

use byteorder::{NativeEndian, ReadBytesExt};

use crate::accd_utils::read_string;

#[derive(Debug, Clone)]
pub struct ACCDTrackData {
    pub track_name: String,
    track_id: i32,
    pub track_meters: i32,
    pub camera_sets: HashMap<String, Vec<String>>,
    pub hud_pages: Vec<String>,
}

impl ACCDTrackData {
    pub fn new(cur: &mut Cursor<&Vec<u8>>) -> Self {
        let track_name = read_string(cur);
        let track_id = cur.read_i32::<NativeEndian>().unwrap();
        let track_meters = cur.read_i32::<NativeEndian>().unwrap();
        let mut camera_sets: HashMap<String, Vec<String>> = HashMap::new();
        let camera_set_count = cur.read_u8().unwrap();

        for _cam_set in 0..camera_set_count {
            let cam_set_name = read_string(cur);
            camera_sets.insert(cam_set_name.clone(), Vec::new());

            let camera_count = cur.read_u8().unwrap();
            let camera_names: &mut Vec<String> = camera_sets.get_mut(&cam_set_name).unwrap();

            for _cam in 0..camera_count {
                let camera_name = read_string(cur);
                camera_names.push(camera_name);
            }
        }

        let mut hud_pages: Vec<String> = Vec::new();
        let hud_pages_count = cur.read_u8().unwrap();

        for _i in 0..hud_pages_count {
            hud_pages.push(read_string(cur));
        }

        ACCDTrackData {
            track_name,
            track_id,
            track_meters,
            camera_sets,
            hud_pages,
        }
    }
}

impl fmt::Display for ACCDTrackData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "/== Track Data ==/")?;
        writeln!(f, "{} ({})", self.track_name, self.track_id)?;
        writeln!(f, "{} meters", self.track_meters)?;
        writeln!(f, "{:#?}", self.camera_sets)?;
        writeln!(f, "{:#?}", self.hud_pages)?;
        writeln!(f, "/----------------/")
    }
}

impl Default for ACCDTrackData {
    fn default() -> Self {
        ACCDTrackData {
            track_name: "".to_string(),
            track_id: -1,
            track_meters: -1,
            camera_sets: HashMap::new(),
            hud_pages: Vec::new(),
        }
    }
}
