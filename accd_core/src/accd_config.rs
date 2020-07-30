use std::fmt;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct ACCDConfig {
    pub debug_sleep_time: u64, //this need to go
    pub protocol_version: u8,
    pub display_name: String,
    pub connection_psw: String,
    pub update_interval: i32,
    pub command_psw: String,
    pub bind_addr: SocketAddr,
    pub destination_addr: SocketAddr,
}

impl Default for ACCDConfig {
    fn default() -> Self {
        ACCDConfig {
            debug_sleep_time: 300,
            protocol_version: 4,
            display_name: String::from("Your name"),
            connection_psw: String::from("asd"),
            update_interval: 250,
            command_psw: String::from(""),
            bind_addr: "0.0.0.0:3400".parse::<SocketAddr>().unwrap(),
            destination_addr: "127.0.0.1:9000".parse::<SocketAddr>().unwrap(),
        }
    }
}

impl fmt::Display for ACCDConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Version: {}\r\n", self.protocol_version)?;
        writeln!(f, "Name: {}\r\n", self.display_name)?;
        writeln!(f, "Interval: {} ms\r\n", self.update_interval)?;
        writeln!(f, "Addr: {}\r\n", self.destination_addr)
    }
}

impl ACCDConfig {
    pub fn new(
        protocol_version: u8,
        display_name: String,
        connection_psw: String,
        update_interval: i32,
        command_psw: String,
        bind_addr: SocketAddr,
        destination_addr: SocketAddr,
    ) -> ACCDConfig {
        ACCDConfig {
            debug_sleep_time: 300,
            protocol_version, //: 4,
            display_name,     //: String::from("ZioYuri78"),
            connection_psw,   //: String::from("asd"),
            update_interval,  //: 250,
            command_psw,      //: String::from(""),
            bind_addr,
            destination_addr, //: "127.0.0.1:9000".parse::<SocketAddr>().unwrap(),
        }
    }
}
