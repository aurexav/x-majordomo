// crates.io
use hap::{
	HapType,
	accessory::HapAccessory,
	characteristic::AsyncCharacteristicCallbacks,
	futures::FutureExt,
	service::{HapService, switch::SwitchService},
};
use serde::{Serialize, Serializer, ser::SerializeStruct};
// self
use crate::{pdu::Pdu, prelude::*};

impl Pdu {
	pub fn switch_of(id: u64, tx: Tx) -> SwitchService {
		let mut s = SwitchService::new(id, Self::ID as _);

		s.power_state.on_update_async(Some(move |_, v| {
			let tx = tx.clone();

			async move {
				const OFF: u8 = 2;
				const ON: u8 = 3;

				let topic = "/yespeed/pdu/yespeed/19847786504205500130392/in/1000101".into();
				let id = id / 7;

				// err
				if v {
					let _ = tx
						.send(MqttMessage {
							topic,
							payload: format!("{{\"devid\":1,\"linid\":{id},\"actid\":{ON}}}"),
						})
						.await;
				} else {
					let _ = tx
						.send(MqttMessage {
							topic,
							payload: format!("{{\"devid\":1,\"linid\":{id},\"actid\":{OFF}}}"),
						})
						.await;
				}

				Ok(())
			}
			.boxed()
		}));

		s
	}
}
impl HapAccessory for Pdu {
	fn get_id(&self) -> u64 {
		self.id
	}

	fn set_id(&mut self, id: u64) {
		self.id = id;
	}

	fn get_service(&self, hap_type: HapType) -> Option<&dyn HapService> {
		self.get_services().into_iter().find(|&service| service.get_type() == hap_type)
	}

	fn get_mut_service(&mut self, hap_type: HapType) -> Option<&mut dyn HapService> {
		self.get_mut_services().into_iter().find(|service| service.get_type() == hap_type)
	}

	fn get_services(&self) -> Vec<&dyn HapService> {
		vec![
			&self.information,
			// &self.voltage,
			&self.switch0,
			&self.switch1,
			&self.switch2,
			&self.switch3,
			&self.switch4,
			&self.switch5,
			&self.switch6,
			&self.switch7,
		]
	}

	fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
		vec![
			&mut self.information,
			// &mut self.voltage,
			&mut self.switch0,
			&mut self.switch1,
			&mut self.switch2,
			&mut self.switch3,
			&mut self.switch4,
			&mut self.switch5,
			&mut self.switch6,
			&mut self.switch7,
		]
	}
}
impl Serialize for Pdu {
	fn serialize<S: Serializer>(&self, serializer: S) -> StdResult<S::Ok, S::Error> {
		let mut s = serializer.serialize_struct("HapAccessory", 2)?;

		s.serialize_field("aid", &self.get_id())?;
		s.serialize_field("services", &self.get_services())?;

		s.end()
	}
}
