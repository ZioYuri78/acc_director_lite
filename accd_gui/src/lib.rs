//#![windows_subsystem = "windows"]
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate accd_core as accd2;
extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;

use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashMap;
use std::net::UdpSocket;
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::error::Error;

use accd2::accd_broadcasting_event::{ACCDBroadcastingEvent, BroadcastingCarEventType};
use accd2::accd_car_info::ACCDCarInfo;
use accd2::accd_config::ACCDConfig;
use accd2::accd_driver_info::ACCDDriverInfo;
use accd2::accd_protocol::ACCDProtocol;
use accd2::accd_protocol::ListenResult;
use accd2::accd_realtime_car_update::ACCDRealtimeCarUpdate;
use accd2::accd_realtime_update::ACCDRealtimeUpdate;
use accd2::accd_registration_result::ACCDRegistrationResult;
use accd2::accd_track_data::ACCDTrackData;
use accd2::accd_utils::parse_config_file;

mod replay_panel;
use crate::replay_panel::ReplayPanel;

mod connection_panel;
use crate::connection_panel::ConnectionPanel;

mod leaderboard_panel;
use crate::leaderboard_panel::LeaderboardPanel;

mod camera_panel;
use crate::camera_panel::CameraPanel;

mod hud_panel;
use crate::hud_panel::HudPanel;

mod track_panel;
use crate::track_panel::TrackPanel;

mod events_panel;
use crate::events_panel::EventsPanel;

mod broadcasting_event_data;
use crate::broadcasting_event_data::BroadcastingEventData;

mod realtime_update_panel;
use crate::realtime_update_panel::RealtimeUpdatePanel;

mod custom_buttons;
use crate::custom_buttons::{CarInfoButton, BroadcastEvtButton};

enum AppTabs {
    ConnTab = 0,
    LeadTab = 1,
    CamTab = 2,
    EvtTab = 3,
    TrackTab = 4,
    RtUpdateTab = 5,
    HudsTab = 6,
    Error,
}

impl From<usize> for AppTabs {
    fn from(value: usize) -> Self {
        match value {
            0 => AppTabs::ConnTab,
            1 => AppTabs::LeadTab,
            2 => AppTabs::CamTab,
            3 => AppTabs::EvtTab,
            4 => AppTabs::TrackTab,
            5 => AppTabs::RtUpdateTab,
            6 => AppTabs::HudsTab,
            _ => AppTabs::Error,
        }
    }
}

#[derive(Debug)]
struct MyChannel(Sender<bool>, Receiver<bool>);
impl Default for MyChannel {
    fn default() -> Self {
        let (a, b) = mpsc::channel();
        MyChannel(a, b)
    }
}



#[derive(Default, NwgUi)]
pub struct MainApp {
    #[nwg_control(size: (800, 600), position: (300, 300), title: "ACC Director LITE", flags: "MAIN_WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [MainApp::on_close], OnInit: [MainApp::init],
        OnResizeBegin: [MainApp::on_resize_begin], OnResizeEnd: [MainApp::on_resize_end], OnResize: [MainApp::resize_tab_container], 
        OnWindowMinimize: [MainApp::resize_tab_container], OnWindowMaximize: [MainApp::resize_tab_container])]
    window: nwg::Window,

    // Tabs 
    #[nwg_control(parent: window, size: (800, 600))]
    tab_container: nwg::TabsContainer,

    // Panels
    #[nwg_partial(parent: tab_container)]
    connection_panel: ConnectionPanel,

    #[nwg_partial(parent: tab_container)]
    #[nwg_events((init_leaderboard_notice, OnNotice):[MainApp::init_leaderboard_tab])]
    leaderboard_panel: LeaderboardPanel,

    #[nwg_partial(parent: tab_container)]
    #[nwg_events((camera_notice, OnNotice):[MainApp::update_camera_tab])]
    camera_panel: CameraPanel,

    #[nwg_partial(parent: tab_container)]
    #[nwg_events((huds_notice, OnNotice):[MainApp::update_huds_tab])]
    huds_panel: HudPanel,

    #[nwg_partial(parent: tab_container)]
    #[nwg_events((replay_10s_btn, OnButtonClick):[MainApp::replay_10s], (replay_30s_btn, OnButtonClick):[MainApp::replay_30s])]
    replay_panel: ReplayPanel,

    #[nwg_partial(parent: tab_container)]
    realtime_update_panel: RealtimeUpdatePanel,

    #[nwg_partial(parent: tab_container)]
    track_panel: TrackPanel,

    /* #[nwg_partial(parent: tab_container)]
    #[nwg_events((broadcasting_event_notice, OnNotice):[MainApp::update_events_tab])]
    broadcasting_events_panel: EventsPanel, */

    config: RefCell<ACCDConfig>,
    accdp: Arc<Mutex<ACCDProtocol>>,
    txrx: Arc<Mutex<MyChannel>>,
}

impl MainApp {
    fn replay_10s(&self) {
        let accdp = self.accdp.lock().unwrap();
        let realtime_update_data = self.realtime_update_panel.realtime_update_data.lock().unwrap().clone();

        let start_time = realtime_update_data.session_time.as_millis() as f32;
        let duration = 10.0;
        let car_index = realtime_update_data.focused_car_index;
        let camera_set = realtime_update_data.active_camera_set;
        let camera = realtime_update_data.active_camera;

        accdp.request_instant_replay(
            start_time - (duration * 1000.0),
            duration * 1000.0,
            car_index,
            camera_set,
            camera,
        );        
    }

    fn replay_30s(&self) {
        let accdp = self.accdp.lock().unwrap();
        let realtime_update_data = self.realtime_update_panel.realtime_update_data.lock().unwrap().clone();

        let start_time = realtime_update_data.session_time.as_millis() as f32;
        let duration = 30.0;
        let car_index = realtime_update_data.focused_car_index;
        let camera_set = realtime_update_data.active_camera_set;
        let camera = realtime_update_data.active_camera;

        accdp.request_instant_replay(
            start_time - (duration * 1000.0),
            duration * 1000.0,
            car_index,
            camera_set,
            camera,
        );
    }

    fn resize_tab_container(&self) {
        self.tab_container
            .set_size(self.window.size().0, self.window.size().1);
    }

    fn on_resize_begin(&self) {
        println!("OnResizeBegin");

        match AppTabs::from(self.tab_container.selected_tab()) {
            AppTabs::CamTab => {
                let camera_handlers = self.camera_panel.camera_handlers.borrow_mut();
                for cam_h in camera_handlers.iter() {
                    nwg::unbind_event_handler(&cam_h);
                }
            }
            AppTabs::HudsTab => {
                let hud_handlers = self.huds_panel.hud_handlers.borrow_mut();
                for hud_h in hud_handlers.iter() {
                    nwg::unbind_event_handler(&hud_h);
                }
            }
            AppTabs::LeadTab => {
                let car_list_handlers = self.leaderboard_panel.car_list_handlers.borrow_mut();
                for car_list_h in car_list_handlers.iter() {
                    nwg::unbind_event_handler(&car_list_h);
                }
            }
            _ => {}
        }
    }

    fn on_resize_end(&self) {
        println!("OnResizeEnd");

        match AppTabs::from(self.tab_container.selected_tab()) {
            AppTabs::ConnTab => {
                //self.connection_grid.fit();
                println!("Connection Tab");
            }
            AppTabs::LeadTab => {
                //self.leaderboard_grid.fit();
                self.init_leaderboard_tab();
                println!("Leaderboard Tab");
            }
            AppTabs::CamTab => {
                //self.cameras_grid.fit();
                self.update_camera_tab();
                println!("Cameras Tab");
            }
            AppTabs::EvtTab => {
                //self.events_grid.fit();
                println!("Events Tab");
            }
            AppTabs::TrackTab => {
                self.track_panel.track_grid.fit();
                println!("Track Tab");
            }
            AppTabs::RtUpdateTab => {
                //self.update_grid.fit();
                println!("Update Tab");
            }
            AppTabs::HudsTab => {
                //self.huds_grid.fit();
                self.update_huds_tab();
                println!("Huds Tab");
            }
            _ => {}
        }
    }

    fn on_close(&self) {
        self.txrx.lock().unwrap().0.send(true).unwrap();
        self.accdp.lock().unwrap().disconnect();

        let camera_handlers = self.camera_panel.camera_handlers.borrow();
        for handler in camera_handlers.iter() {
            nwg::unbind_event_handler(&handler);
        }

        let hud_handlers = self.huds_panel.hud_handlers.borrow();
        for handler in hud_handlers.iter() {
            nwg::unbind_event_handler(&handler);
        }

        let car_list_handlers = self.leaderboard_panel.car_list_handlers.borrow();
        for handler in car_list_handlers.iter() {
            nwg::unbind_event_handler(&handler);
        }
        nwg::stop_thread_dispatch();
    }


    fn update_camera_tab(&self) {
        let camera_sets = self.camera_panel.camera_data.lock().unwrap().clone();
        self.camera_panel.camera_buttons.borrow_mut().clear();
        self.camera_panel.camera_handlers.borrow_mut().clear();

        for (k, v) in camera_sets {
            for c in &v {
                let accdp = self.accdp.clone();
                let mut new_button = Default::default();
                nwg::Button::builder()
                    .text(&c)
                    .parent(&self.camera_panel.camera_tab)
                    .build(&mut new_button)
                    .expect("Failed to build button");

                let mut buttons = self.camera_panel.camera_buttons.borrow_mut();
                let mut handlers = self.camera_panel.camera_handlers.borrow_mut();

                let blen = buttons.len() as u32;
                let (x, y) = (blen % 6, blen / 6);
                self.camera_panel.camera_grid.add_child(x, y, &new_button);

                let k2 = k.clone();
                let c2 = c.clone();

                let new_button_handle = new_button.handle;
                let handler = nwg::bind_event_handler(
                    &new_button.handle,
                    &self.camera_panel.camera_tab.handle,
                    move |evt, _evt_data, handle| match evt {
                        nwg::Event::OnButtonClick => {
                            if handle == new_button_handle {
                                accdp.lock().unwrap().set_camera(k2.clone(), c2.clone());
                            }
                        }
                        _ => {}
                    },
                );

                buttons.push(new_button);
                handlers.push(handler);
            }
        }
    }

    fn update_huds_tab(&self) {
        
        let huds = self.huds_panel.huds_data.lock().unwrap().clone();
        self.huds_panel.hud_buttons.borrow_mut().clear();
        self.huds_panel.hud_handlers.borrow_mut().clear();

        for h in huds {
            let accdp = self.accdp.clone();
            let mut new_button = Default::default();
            nwg::Button::builder()
                .text(&h)
                .parent(&self.huds_panel.huds_tab)
                .build(&mut new_button)
                .expect("Failed to build button");

            let mut buttons = self.huds_panel.hud_buttons.borrow_mut();
            let mut handlers = self.huds_panel.hud_handlers.borrow_mut();

            let blen = buttons.len() as u32;
            let (x, y) = (blen % 3, blen / 3);
            self.huds_panel.huds_grid.add_child(x, y, &new_button);

            let h2 = h.clone();

            let new_button_handle = new_button.handle;
            let handler = nwg::bind_event_handler(
                &new_button.handle,
                &self.huds_panel.huds_tab.handle,
                move |evt, _evt_data, handle| match evt {
                    nwg::Event::OnButtonClick => {
                        if handle == new_button_handle {
                            accdp.lock().unwrap().request_hud_page(h2.clone());
                        }
                    }
                    _ => {}
                },
            );

            buttons.push(new_button);
            handlers.push(handler);
        }
    }

    
    fn init_leaderboard_tab(&self) {
        let entry_list_cars = self.leaderboard_panel.entry_list_cars_data.lock().unwrap();
        let mut row = 0;
        if entry_list_cars.len() > 0 {
            
            self.leaderboard_panel.car_list_buttons.borrow_mut().clear();
            self.leaderboard_panel.car_list_handlers.borrow_mut().clear();

            for c in entry_list_cars.clone() {
                let accdp = self.accdp.clone();
                let mut new_button = Default::default();

                let driver;
                if c.drivers.len() > 0 {
                    driver = c
                        .drivers
                        .get(c.current_driver_index as usize)
                        .unwrap()
                        .clone();
                } else {
                    driver = ACCDDriverInfo::default();
                }

                CarInfoButton::builder()
                    .parent(&self.leaderboard_panel.leaderboard_tab)
                    .build(&mut new_button)
                    .expect("Failed to build CarInfoButton");

                new_button.car_info = c.clone();
                new_button.set_text(&format!(
                    "#{} | {} | {} {}",
                    &c.race_number, &c.team_name, &driver.first_name, &driver.last_name,
                ));

                
                let mut buttons = self.leaderboard_panel.car_list_buttons.borrow_mut();
                let mut handlers = self.leaderboard_panel.car_list_handlers.borrow_mut();

                self.leaderboard_panel.leaderboard_grid.add_child(0, row, &new_button);
                row = row + 1;

                let new_button_handle = new_button.handle;
                let handler = nwg::bind_event_handler(
                    &new_button_handle,
                    &self.leaderboard_panel.leaderboard_tab.handle,
                    move |evt, _evt_data, handle| match evt {
                        nwg::Event::OnButtonClick => {
                            if handle == new_button_handle {
                                accdp.lock().unwrap().set_focus(
                                    Some(c.car_index),
                                    "".to_string(),
                                    "".to_string(),
                                );
                            }
                        }
                        _ => {}
                    },
                );

                buttons.push(new_button);
                handlers.push(handler);
            }
        }
    }


    fn sort_leaderboard(&self) {
        /*  let mut car_buttons = self.car_list_buttons.borrow_mut();
        if car_buttons.len() > 0 {
            car_buttons.sort_by(|a,b|a.rt_update.position.cmp(&b.rt_update.position));
            /* while self.leaderboard_grid.

            } */
        }
        unimplemented!("sort leaderboard"); */
        println!("invalidate");
        self.window.invalidate();
    }

    fn update_events_tab(&self) {
        let mut buttons = self.broadcasting_events_panel.broadcast_evt_buttons.borrow_mut();
        let mut handlers = self.broadcasting_events_panel.broadcast_evt_handlers.borrow_mut();

        if buttons.len() > 10 {            
            let btn = buttons.remove(0);
            handlers.remove(0);
            self.broadcasting_events_panel.events_grid.remove_child::<BroadcastEvtButton>(btn);
            thread::sleep(Duration::from_secs(1));
            self.window.invalidate();
        }

        let event_data = self.broadcasting_events_panel.broadcasting_event_data.lock().unwrap().clone();
        let mut new_button = Default::default();
        BroadcastEvtButton::builder().parent(&self.broadcasting_events_panel.events_tab).build(&mut new_button).expect("Failed to build BroadcastEvtButton");        
        new_button.broadcast_evt = event_data.event.clone();
        new_button.set_text(&new_button.broadcast_evt.event_msg);
        

        let row = buttons.len() as u32;
        self.broadcasting_events_panel.events_grid.add_child(0, row, &new_button);
        
        let accdp = self.accdp.clone();
        let new_button_handle = new_button.handle;
        let handler = nwg::bind_event_handler(
            &new_button_handle,
            &self.broadcasting_events_panel.events_tab.handle,
            move |evt, _evt_data, handle| match evt {
                nwg::Event::OnButtonClick => {
                    if handle == new_button_handle {
                        let start_time = (event_data.event.event_time_ms as f32) - event_data.replay_seconds_back;
                        let duration= event_data.replay_duration;
                        let car_index= event_data.event.event_car_id;
                        accdp.lock().unwrap().request_instant_replay(start_time, duration, car_index, "".to_string(), "".to_string());
                    }
                },
                _ => {}
            }
        );

        buttons.push(new_button);
        handlers.push(handler);
    }

    fn init(&self) {
        let c_trtx = self.txrx.clone();
        *self.accdp.lock().unwrap() =
            ACCDProtocol::new(parse_config_file("./config/default.cfg".to_string()));
        let c_accdp = Arc::clone(&self.accdp);            

        let bind_addr = c_accdp.lock().unwrap().config.bind_addr;
        c_accdp.lock().unwrap().socket = Some(RefCell::new(UdpSocket::bind(bind_addr).unwrap()));

        c_accdp.lock().unwrap().request_connection();

        let track_notice = self.track_panel.track_notice.sender();
        let track_data = Arc::clone(&self.track_panel.track_data);

        let camera_notice = self.camera_panel.camera_notice.sender();
        let camera_data = Arc::clone(&self.camera_panel.camera_data);

        let huds_notice = self.huds_panel.huds_notice.sender();
        let huds_data = Arc::clone(&self.huds_panel.huds_data);

        let registration_notice = self.connection_panel.registration_notice.sender();
        let registration_data = Arc::clone(&self.connection_panel.registration_data);

        let realtime_notice = self.realtime_update_panel.realtime_update_notice.sender();
        let realtime_update_data = Arc::clone(&self.realtime_update_panel.realtime_update_data);

        let init_leaderboard_notice = self.leaderboard_panel.init_leaderboard_notice.sender();
        let entry_list_cars_data = Arc::clone(&self.leaderboard_panel.entry_list_cars_data);

        let update_leaderboard_notice = self.leaderboard_panel.update_leaderboard_notice.sender();
       
        let update_car_data = Arc::clone(&self.leaderboard_panel.update_car_data);

        let broadcasting_event_notice = self.broadcasting_events_panel.broadcasting_event_notice.sender();
        let broadcasting_event_data = Arc::clone(&self.broadcasting_events_panel.broadcasting_event_data);        
        

        thread::spawn(move || loop {
            match c_trtx.lock().unwrap().1.try_recv() {
                Ok(data) => {
                    if data == true {
                        println!("OK: {:?}", data);
                        break;
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(e) => {
                    println!("ERROR: {}", e);
                }
            };

            let listen_result = c_accdp.lock().unwrap().listen_step();

            match listen_result {
                ListenResult::RegistrationResult(reg_result) => {
                    *registration_data.lock().unwrap() = reg_result.clone();
                    registration_notice.notice();
                }
                ListenResult::TrackData(trk_data) => {
                    *track_data.lock().unwrap() = trk_data.clone();
                    track_notice.notice();

                    *camera_data.lock().unwrap() = trk_data.camera_sets.clone();
                    camera_notice.notice();

                    *huds_data.lock().unwrap() = trk_data.hud_pages.clone();
                    huds_notice.notice();
                }
                ListenResult::RealTimeUpdate(rtu) => {
                    *realtime_update_data.lock().unwrap() = rtu.clone();                    
                    realtime_notice.notice();
                }
                ListenResult::RealTimeCarUpdate(rt_car_update) => {
                    thread::sleep(Duration::from_millis(1));                    
                    *update_car_data.lock().unwrap() = rt_car_update.clone();
                    update_leaderboard_notice.notice();
                }
                ListenResult::EntryList(entry_list_car) => {
                    thread::sleep(Duration::from_millis(1));
                    *entry_list_cars_data.lock().unwrap() = entry_list_car.clone();
                    init_leaderboard_notice.notice();
                }
                ListenResult::EntryListCar(car_info) => {
                    match entry_list_cars_data
                        .lock()
                        .unwrap()
                        .iter_mut()
                        .find(|car| car.car_index == car_info.car_index)
                    {
                        Some(car) => {
                            *car = car_info.clone();
                        }
                        None => {}
                    }
                }
                ListenResult::BroadcastingEvent(broadcasting_event) => {
                    
                    match broadcasting_event.event_type {
                        //==============================================================================
                        // Just for testing purpose, remove later
                        BroadcastingCarEventType::LapCompleted => {
                            let data = BroadcastingEventData {
                                event: broadcasting_event.clone(),
                                replay_seconds_back: 10.0,
                                replay_duration: 10.0,
                            };
                            *broadcasting_event_data.lock().unwrap() = data;
                            broadcasting_event_notice.notice();
                        } 
                        //==============================================================================
                        BroadcastingCarEventType::BestSessionLap => {                                                                         
                            // TODO: maybe search some String-to-Duration crate       
                            let data = BroadcastingEventData {
                                event: broadcasting_event.clone(),
                                replay_seconds_back: 10.0,
                                replay_duration: 10.0,
                            };
                            *broadcasting_event_data.lock().unwrap() = data;
                            broadcasting_event_notice.notice();                    
                        }
                        BroadcastingCarEventType::Accident => {
                            let data = BroadcastingEventData {
                                event: broadcasting_event.clone(),
                                replay_seconds_back: 10.0,
                                replay_duration: 10.0,
                            };
                            *broadcasting_event_data.lock().unwrap() = data;
                            broadcasting_event_notice.notice();   
                        }
                        BroadcastingCarEventType::PenaltyCommMsg => {
                            let data = BroadcastingEventData {
                                event: broadcasting_event.clone(),
                                replay_seconds_back: 10.0,
                                replay_duration: 6.0,
                            };
                            *broadcasting_event_data.lock().unwrap() = data;
                            broadcasting_event_notice.notice();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        });
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mt = thread::Builder::new().stack_size(10 * 1024 * 1024);

    let hmt = mt.spawn(|| {
        nwg::init().expect("Failed to init Native Windows GUI");
        nwg::Font::set_global_family("Segoe UI").expect("Failed to set defautl font");
        let _app = MainApp::build_ui(Default::default()).expect("Failed to build UI");
        nwg::dispatch_thread_events(); //_with_callback(|| {});
    });

    hmt.unwrap().join().unwrap();

    Ok(())
}
