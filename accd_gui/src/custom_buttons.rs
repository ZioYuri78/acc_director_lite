use nwd::NwgUi;
use nwg::NativeUi;

use accd2::accd_broadcasting_event::{ACCDBroadcastingEvent, BroadcastingCarEventType};
use accd2::accd_car_info::ACCDCarInfo;
use accd2::accd_realtime_car_update::ACCDRealtimeCarUpdate;

#[derive(Default)]
pub struct CarInfoButton {
    pub base: nwg::Button,
    pub car_info: ACCDCarInfo,
    pub rt_update: ACCDRealtimeCarUpdate,
}

nwg::subclass_control!(CarInfoButton, Button, base);

impl CarInfoButton {
    pub fn builder<'a>() -> CarInfoButtonBuilder<'a> {
        CarInfoButtonBuilder {
            button_builder: nwg::Button::builder().text("CarInfoButton with builder"),
            car_info: ACCDCarInfo::default(),
            rt_update: ACCDRealtimeCarUpdate::default(),
        }
    }
}

pub struct CarInfoButtonBuilder<'a> {
    button_builder: nwg::ButtonBuilder<'a>,
    car_info: ACCDCarInfo,
    rt_update: ACCDRealtimeCarUpdate,
}

impl<'a> CarInfoButtonBuilder<'a> {
    pub fn data(mut self, ci: ACCDCarInfo, rt: ACCDRealtimeCarUpdate) -> CarInfoButtonBuilder<'a> {
        self.car_info = ci;
        self.rt_update = rt;
        self
    }

    pub fn parent<C: Into<nwg::ControlHandle>>(mut self, p: C) -> CarInfoButtonBuilder<'a> {
        self.button_builder = self.button_builder.parent(p);
        self
    }

    pub fn build(self, btn: &mut CarInfoButton) -> Result<(), nwg::NwgError> {
        self.button_builder.build(&mut btn.base)?;
        btn.car_info = self.car_info;
        btn.rt_update = self.rt_update;
        Ok(())
    }
}

#[derive(Default)]
pub struct BroadcastEvtButton {
    pub base: nwg::Button,
    pub broadcast_evt: ACCDBroadcastingEvent,
}

nwg::subclass_control!(BroadcastEvtButton, Button, base);

impl BroadcastEvtButton {
    pub fn builder<'a>() -> BroadcastEvtButtonBuilder<'a> {
        BroadcastEvtButtonBuilder {
            button_builder: nwg::Button::builder().text("BroadcastEvtButton with builder"),
            broadcast_evt: ACCDBroadcastingEvent::default(),
        }
    }
}

impl Into<nwg::ControlHandle> for BroadcastEvtButton {
    fn into(self) -> nwg::ControlHandle {
        self.base.handle
    }
}

pub struct BroadcastEvtButtonBuilder<'a> {
    button_builder: nwg::ButtonBuilder<'a>,
    broadcast_evt: ACCDBroadcastingEvent,
}

impl<'a> BroadcastEvtButtonBuilder<'a> {
    pub fn data(mut self, ci: ACCDBroadcastingEvent) -> BroadcastEvtButtonBuilder<'a> {
        self.broadcast_evt = ci;
        self
    }

    pub fn parent<C: Into<nwg::ControlHandle>>(mut self, p: C) -> BroadcastEvtButtonBuilder<'a> {
        self.button_builder = self.button_builder.parent(p);
        self
    }

    pub fn build(self, btn: &mut BroadcastEvtButton) -> Result<(), nwg::NwgError> {
        self.button_builder.build(&mut btn.base)?;
        btn.broadcast_evt = self.broadcast_evt;
        Ok(())
    }
}
