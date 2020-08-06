extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgPartial;

use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use accd2::accd_car_info::ACCDCarInfo;
use accd2::accd_driver_info::ACCDDriverInfo;
use accd2::accd_realtime_car_update::ACCDRealtimeCarUpdate;

use crate::custom_buttons::CarInfoButton;

#[derive(Default, NwgPartial)]
pub struct LeaderboardPanel {
    #[nwg_control(text: "Leaderboard")]
    pub leaderboard_tab: nwg::Tab,

    #[nwg_layout(parent: leaderboard_tab, spacing: 1, max_row: Some(30))]
    pub leaderboard_grid: nwg::GridLayout,

    #[nwg_control]
    pub init_leaderboard_notice: nwg::Notice,

    #[nwg_control]
    #[nwg_events(OnNotice:[LeaderboardPanel::update_leaderboard_tab])]
    pub update_leaderboard_notice: nwg::Notice,

    pub update_car_data: Arc<Mutex<ACCDRealtimeCarUpdate>>,
    pub entry_list_cars_data: Arc<Mutex<Vec<ACCDCarInfo>>>,
    pub car_list_buttons: RefCell<Vec<CarInfoButton>>,
    pub car_list_handlers: RefCell<Vec<nwg::EventHandler>>,
}

impl LeaderboardPanel {
    fn update_leaderboard_tab(&self) {
        let car_update = self.update_car_data.lock().unwrap();
        let mut car_buttons = self.car_list_buttons.borrow_mut();

        match car_buttons
            .iter_mut()
            .find(|car_upt| car_upt.car_info.car_index == car_update.car_index as u16)
        {
            Some(btn) => {
                if btn.rt_update.position != -1 && btn.rt_update.position != car_update.position {
                    self.leaderboard_grid.move_child_by_pos::<CarInfoButton>(0, (car_update.position - 1) as u32, 0, (btn.rt_update.position - 1) as u32);
                    self.leaderboard_grid.move_child(&btn.handle, 0, (car_update.position - 1) as u32);                    
                    btn.rt_update = car_update.clone();
                    self.leaderboard_grid.fit();
                    return;
                }

                let current_driver;
                match btn
                    .car_info
                    .drivers
                    .get(btn.car_info.current_driver_index as usize)
                {
                    Some(drv) => {
                        current_driver = drv.clone();
                    }
                    None => {
                        current_driver = ACCDDriverInfo::default();
                    }
                }

                btn.rt_update = car_update.clone();
                btn.set_text(&format!(
                    "P{}({}) | #{} | {} | {} {} | {}",
                    btn.rt_update.position,
                    btn.rt_update.track_position,
                    btn.car_info.race_number,
                    btn.car_info.team_name,
                    current_driver.first_name,
                    current_driver.last_name,
                    btn.rt_update.delta
                ));
            }
            None => {}
        }
    }
}
