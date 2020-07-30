use std::cell::RefCell;
use std::io::Cursor;
use std::net::UdpSocket;
use std::time::Instant;

use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};

use crate::accd_broadcasting_event::ACCDBroadcastingEvent;
use crate::accd_car_info::ACCDCarInfo;
use crate::accd_config::ACCDConfig;
use crate::accd_driver_info::ACCDDriverInfo;
use crate::accd_enums::NationalityEnum;
use crate::accd_realtime_car_update::ACCDRealtimeCarUpdate;
use crate::accd_realtime_update::ACCDRealtimeUpdate;
use crate::accd_registration_result::ACCDRegistrationResult;
use crate::accd_track_data::ACCDTrackData;

use crate::accd_utils::{read_string, write_string};

#[derive(Debug)]
enum InboundMessageTypes {
    RegistrationResult = 1,
    RealTimeUpdate = 2,
    RealTimeCarUpdate = 3,
    EntryList = 4,
    TrackData = 5,
    EntryListCar = 6,
    BroadcastingEvent = 7,
    Error,
}

impl From<u8> for InboundMessageTypes {
    fn from(value: u8) -> Self {
        match value {
            1 => InboundMessageTypes::RegistrationResult,
            2 => InboundMessageTypes::RealTimeUpdate,
            3 => InboundMessageTypes::RealTimeCarUpdate,
            4 => InboundMessageTypes::EntryList,
            5 => InboundMessageTypes::TrackData,
            6 => InboundMessageTypes::EntryListCar,
            7 => InboundMessageTypes::BroadcastingEvent,
            _ => InboundMessageTypes::Error,
        }
    }
}

#[derive(Debug)]
enum OutboundMessageTypes {
    RegisterCommandApplication = 1,
    UnregisterCommandApplication = 9,
    RequestEntryList = 10,
    RequestTrackData = 11,
    ChangeHudPage = 49,
    ChangeFocus = 50,
    InstantReplayRequest = 51,
    PlayManualReplayHighlight = 52, // TODO, but planned
    SaveManualReplayHighlight = 60, // TODO, but planned: saving manual replays gives distributed clients the possibility to see the play the same replay
    Error,
}

impl From<u8> for OutboundMessageTypes {
    fn from(value: u8) -> Self {
        match value {
            1 => OutboundMessageTypes::RegisterCommandApplication,
            9 => OutboundMessageTypes::UnregisterCommandApplication,
            10 => OutboundMessageTypes::RequestEntryList,
            11 => OutboundMessageTypes::RequestTrackData,
            49 => OutboundMessageTypes::ChangeHudPage,
            50 => OutboundMessageTypes::ChangeFocus,
            51 => OutboundMessageTypes::InstantReplayRequest,
            52 => OutboundMessageTypes::PlayManualReplayHighlight,
            60 => OutboundMessageTypes::SaveManualReplayHighlight,
            _ => OutboundMessageTypes::Error,
        }
    }
}

pub enum ListenResult {
    RegistrationResult(ACCDRegistrationResult),
    RealTimeUpdate(ACCDRealtimeUpdate),
    RealTimeCarUpdate(ACCDRealtimeCarUpdate),
    EntryList(Vec<ACCDCarInfo>),
    TrackData(ACCDTrackData),
    EntryListCar(ACCDCarInfo),
    BroadcastingEvent(ACCDBroadcastingEvent),
    Error,
}

#[derive(Debug)]
pub struct ACCDProtocol {
    pub config: ACCDConfig,
    pub socket: Option<RefCell<UdpSocket>>,
    registration_result: ACCDRegistrationResult,

    message_type: u8,

    pub(crate) entry_list_cars: Vec<ACCDCarInfo>,

    last_entry_list_request: Instant,
}

impl Default for ACCDProtocol {
    fn default() -> Self {
        ACCDProtocol {
            config: ACCDConfig::default(),
            socket: None,
            registration_result: ACCDRegistrationResult::default(),
            message_type: 1,
            entry_list_cars: Vec::new(),
            last_entry_list_request: Instant::now(),
        }
    }
}

impl ACCDProtocol {
    pub fn new(config: ACCDConfig) -> Self {
        ACCDProtocol {
            config,
            ..ACCDProtocol::default()
        }
    }

    pub fn request_connection(&self) {
        let mut buffer = Vec::new();
        buffer.write_u8(self.message_type).unwrap();
        buffer.write_u8(self.config.protocol_version).unwrap();
        write_string(&mut buffer, &self.config.display_name);

        write_string(&mut buffer, &self.config.connection_psw);

        buffer
            .write_i32::<NativeEndian>(self.config.update_interval)
            .unwrap();

        write_string(&mut buffer, &self.config.command_psw);

        match self
            .socket
            .as_ref()
            .unwrap()
            .borrow()
            .send_to(&buffer, self.config.destination_addr)
        {
            Ok(bytes) => {
                println!("=== Request connection ({} bytes) ===", bytes);
            }

            Err(e) => {
                panic!("ERROR: {}", e);
            }
        }
    }

    pub fn disconnect(&self) {
        let mut buffer = Vec::new();
        buffer
            .write_u8(OutboundMessageTypes::UnregisterCommandApplication as u8)
            .unwrap();
        buffer
            .write_i32::<NativeEndian>(self.registration_result.connection_id)
            .unwrap();

        match self
            .socket
            .as_ref()
            .unwrap()
            .borrow()
            .send_to(&buffer, self.config.destination_addr)
        {
            Ok(bytes) => {
                println!("=== Disconnect ({} bytes) ===", bytes);
            }

            Err(e) => {
                println!("ERROR: {}", e);
            }
        }
    }

    fn request_entry_list(&self) {
        let mut buffer = Vec::new();
        buffer
            .write_u8(OutboundMessageTypes::RequestEntryList as u8)
            .unwrap();
        buffer
            .write_i32::<NativeEndian>(self.registration_result.connection_id)
            .unwrap();

        match self
            .socket
            .as_ref()
            .unwrap()
            .borrow()
            .send_to(&buffer, &self.config.destination_addr)
        {
            Ok(bytes) => {
                println!("=== Request entry list ({} bytes) ===", bytes);
            }

            Err(e) => {
                println!("ERROR: {}", e);
            }
        }
    }

    fn request_track_data(&self) {
        let mut buffer = Vec::new();
        buffer
            .write_u8(OutboundMessageTypes::RequestTrackData as u8)
            .unwrap();
        buffer
            .write_i32::<NativeEndian>(self.registration_result.connection_id)
            .unwrap();

        match self
            .socket
            .as_ref()
            .unwrap()
            .borrow()
            .send_to(&buffer, self.config.destination_addr)
        {
            Ok(bytes) => {
                println!("=== Request track data ({} bytes) ===", bytes);
            }

            Err(e) => {
                println!("ERROR: {}", e);
            }
        };
    }

    pub fn set_camera(&self, camera_set: String, camera: String) {
        self.set_focus_internal(None, camera_set, camera);
    }

    pub fn set_focus(&self, car_index: Option<u16>, camera_set: String, camera: String) {
        self.set_focus_internal(car_index, camera_set, camera);
    }

    fn set_focus_internal(&self, car_index: Option<u16>, camera_set: String, camera: String) {
        let mut buffer = Vec::new();
        buffer
            .write_u8(OutboundMessageTypes::ChangeFocus as u8)
            .unwrap();
        buffer
            .write_i32::<NativeEndian>(self.registration_result.connection_id)
            .unwrap();

        if car_index == None {
            buffer.write_u8(0u8).unwrap();
        } else {
            buffer.write_u8(1u8).unwrap();
            buffer
                .write_u16::<NativeEndian>(car_index.unwrap())
                .unwrap();
        }

        if camera_set.is_empty() || camera.is_empty() {
            buffer.write_u8(0u8).unwrap();
        } else {
            buffer.write_u8(1u8).unwrap();
            write_string(&mut buffer, &camera_set);
            write_string(&mut buffer, &camera);
        }

        match self
            .socket
            .as_ref()
            .unwrap()
            .borrow()
            .send_to(&buffer, self.config.destination_addr)
        {
            Ok(bytes) => {
                println!(
                    "=== Set focus | {:?} | {} | {} ({} bytes) ===",
                    car_index, camera_set, camera, bytes
                );
            }

            Err(e) => {
                println!("ERROR: {}", e);
            }
        };
    }

    pub fn request_instant_replay(
        &self,
        start_session_time: f32,
        duration_ms: f32,
        initial_focused_car_index: i32,
        initial_camera_set: String,
        initial_camera: String,
    ) {
        let mut buffer = Vec::new();
        buffer
            .write_u8(OutboundMessageTypes::InstantReplayRequest as u8)
            .unwrap();
        buffer
            .write_i32::<NativeEndian>(self.registration_result.connection_id)
            .unwrap();
        buffer
            .write_f32::<NativeEndian>(start_session_time)
            .unwrap();
        buffer.write_f32::<NativeEndian>(duration_ms).unwrap();
        buffer
            .write_i32::<NativeEndian>(initial_focused_car_index)
            .unwrap();
        write_string(&mut buffer, &initial_camera_set);
        write_string(&mut buffer, &initial_camera);

        match self
            .socket
            .as_ref()
            .unwrap()
            .borrow()
            .send_to(&buffer, self.config.destination_addr)
        {
            Ok(bytes) => {
                println!("=== Request Instant Replay ({} bytes)==", bytes);
            }
            Err(e) => {
                println!("ERROR: {}", e);
            }
        }
    }

    pub fn request_hud_page(&self, hud_page: String) {
        let mut buffer = Vec::new();
        buffer
            .write_u8(OutboundMessageTypes::ChangeHudPage as u8)
            .unwrap();
        buffer
            .write_i32::<NativeEndian>(self.registration_result.connection_id)
            .unwrap();
        write_string(&mut buffer, &hud_page);

        match self
            .socket
            .as_ref()
            .unwrap()
            .borrow()
            .send_to(&buffer, self.config.destination_addr)
        {
            Ok(bytes) => {
                println!("=== Request HUD Page | {} ({} bytes)", hud_page, bytes);
            }
            Err(e) => {
                println!("ERROR: {}", e);
            }
        }
    }

    pub fn listen_step(&mut self) -> ListenResult {
        let mut msg = [0; 2048];
        match self.socket.as_ref().unwrap().borrow().recv_from(&mut msg) {
            Ok(_) => {}

            Err(e) => {
                panic!("ERROR: {}", e);
            }
        }

        let msg = msg.to_vec();
        let mut cur = Cursor::new(&msg);
        let message_type = InboundMessageTypes::from(cur.read_u8().unwrap());

        match message_type {
            InboundMessageTypes::RegistrationResult => {
                self.registration_result = ACCDRegistrationResult::new(&mut cur);
                self.request_track_data();
                self.request_entry_list();
                ListenResult::RegistrationResult(self.registration_result.clone())
            }

            InboundMessageTypes::RealTimeUpdate => {
                ListenResult::RealTimeUpdate(ACCDRealtimeUpdate::new(&mut cur))
            }

            InboundMessageTypes::RealTimeCarUpdate => {
                let car_index = cur.read_u16::<NativeEndian>().unwrap() as i32;
                let driver_index = cur.read_u16::<NativeEndian>().unwrap() as i32;
                let driver_count = cur.read_u8().unwrap();

                match self
                    .entry_list_cars
                    .iter()
                    .find(|car_info| car_info.car_index == car_index as u16)
                {
                    Some(car_info) => {
                        if car_info.drivers.len() != driver_count as usize {
                            if Instant::now()
                                .saturating_duration_since(self.last_entry_list_request)
                                .as_secs()
                                > 1
                            {
                                self.last_entry_list_request = Instant::now();
                                self.request_entry_list();
                                println!(
                                    "CarUpdate {}|{} not know, will ask for new EntryList",
                                    car_index, driver_index
                                );
                            };
                        } else {
                            let real_time_car_update = ACCDRealtimeCarUpdate::new(
                                &mut cur,
                                car_index,
                                driver_index,
                                driver_count,
                            );
                            return ListenResult::RealTimeCarUpdate(real_time_car_update);
                        }
                    }

                    None => {}
                };

                ListenResult::Error
            }

            InboundMessageTypes::EntryList => {
                self.entry_list_cars.clear();
                let connection_id = cur.read_i32::<NativeEndian>().unwrap();

                if connection_id == self.registration_result.connection_id {
                    let car_entry_count = cur.read_u16::<NativeEndian>().unwrap();

                    for _i in 0..car_entry_count {
                        self.entry_list_cars
                            .push(ACCDCarInfo::new(cur.read_u16::<NativeEndian>().unwrap()));
                    }
                }

                ListenResult::EntryList(self.entry_list_cars.clone())
            }

            InboundMessageTypes::TrackData => {
                let connection_id = cur.read_i32::<NativeEndian>().unwrap();

                if connection_id == self.registration_result.connection_id {
                    ListenResult::TrackData(ACCDTrackData::new(&mut cur))
                } else {
                    ListenResult::TrackData(ACCDTrackData::default())
                }
            }

            InboundMessageTypes::EntryListCar => {
                let car_index = cur.read_u16::<NativeEndian>().unwrap();
                let mut car_info = ACCDCarInfo::default();

                match self
                    .entry_list_cars
                    .iter_mut()
                    .find(|car_info| car_info.car_index == car_index)
                {
                    Some(element) => {
                        element.car_model_type = cur.read_u8().unwrap();
                        element.team_name = read_string(&mut cur);
                        element.race_number = cur.read_i32::<NativeEndian>().unwrap();
                        element.cup_category = cur.read_u8().unwrap();
                        element.current_driver_index = cur.read_u8().unwrap() as i32;
                        element.nationality =
                            NationalityEnum::from(cur.read_u16::<NativeEndian>().unwrap() as u8);

                        element.drivers = Vec::new();
                        let drivers_car_count = cur.read_u8().unwrap();

                        for _i in 0..drivers_car_count {
                            element.drivers.push(ACCDDriverInfo::new(&mut cur));
                        }

                        car_info = element.clone();
                    }
                    None => {
                        println!("Entry list update for unknow car_index {}", car_index);
                    }
                }

                ListenResult::EntryListCar(car_info)
            }

            InboundMessageTypes::BroadcastingEvent => {
                let broadcasting_event = ACCDBroadcastingEvent::new(&mut cur, &self);
                ListenResult::BroadcastingEvent(broadcasting_event)
            }

            InboundMessageTypes::Error => ListenResult::Error,
        }
    }
}
