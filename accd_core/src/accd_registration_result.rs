use std::fmt;
use std::io::Cursor;

use byteorder::{NativeEndian, ReadBytesExt};

#[derive(Debug, Clone)]
pub struct ACCDRegistrationResult {
    pub connection_id: i32,
    pub connection_success: u8,
    pub is_read_only: u8,
    pub err_msg: String,
}

impl ACCDRegistrationResult {
    pub fn new(cur: &mut Cursor<&Vec<u8>>) -> Self {
        ACCDRegistrationResult {
            connection_id: cur.read_i32::<NativeEndian>().unwrap(),
            connection_success: cur.read_u8().unwrap(),
            is_read_only: cur.read_u8().unwrap(),
            err_msg: match String::from_utf8(cur.get_mut().to_vec()) {
                Ok(data) => data,
                Err(e) => {
                    println!("ERROR: {}", e);
                    "".to_string()
                }
            }, /* .unwrap()
               .trim()
               .to_string(), */
        }
    }
}

impl fmt::Display for ACCDRegistrationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "/== Registration Result ==/")?;
        writeln!(f, "Id: {}", self.connection_id)?;
        writeln!(f, "Success: {}", self.connection_success)?;
        writeln!(f, "Read Only: {}", self.is_read_only)?;
        writeln!(f, "Error: {}", self.err_msg)?;
        writeln!(f, "/-------------------------/")
    }
}

impl Default for ACCDRegistrationResult {
    fn default() -> Self {
        ACCDRegistrationResult {
            connection_id: -1,
            connection_success: 0,
            is_read_only: 0,
            err_msg: String::from("default"),
        }
    }
}
