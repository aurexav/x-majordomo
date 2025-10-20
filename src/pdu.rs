mod hap;
mod mqtt;

// std
use std::future::Future;
// crates.io
use ::hap::{
	HapType,
	accessory::AccessoryInformation,
	server::{IpServer, Server},
	service::{accessory_information::AccessoryInformationService, switch::SwitchService},
};
use tokio::{
	sync::{mpsc, oneshot},
	task,
};
// self
use crate::prelude::*;

#[derive(Debug)]
struct Pdu {
	id: u64,
	information: AccessoryInformationService,
	// voltage: ?,
	switch0: SwitchService,
	switch1: SwitchService,
	switch2: SwitchService,
	switch3: SwitchService,
	switch4: SwitchService,
	switch5: SwitchService,
	switch6: SwitchService,
	switch7: SwitchService,
}
impl Pdu {
	const ID: u8 = 1;
	const NAME: &'static str = "Pdu";

	pub fn new(tx: Tx) -> Self {
		Self {
			id: Pdu::ID as _,
			information: AccessoryInformation { name: Pdu::NAME.into(), ..Default::default() }
				.to_service(1, Pdu::ID as _)
				.unwrap(),
			// voltage: TemperatureSensorService::new(63, Pdu::ID as _),
			switch0: Pdu::switch_of(7, tx.clone()),
			switch1: Pdu::switch_of(14, tx.clone()),
			switch2: Pdu::switch_of(21, tx.clone()),
			switch3: Pdu::switch_of(28, tx.clone()),
			switch4: Pdu::switch_of(35, tx.clone()),
			switch5: Pdu::switch_of(42, tx.clone()),
			switch6: Pdu::switch_of(49, tx.clone()),
			switch7: Pdu::switch_of(56, tx),
		}
	}
}

pub async fn initialize(
	mqtt_host: String,
	bridge: &IpServer,
) -> Result<impl Future<Output = Result<()>>> {
	let (hap_tx, mqtt_rx) = mpsc::channel(8);
	let (mqtt_tx, hap_rx) = oneshot::channel();
	let ptr = bridge.add_accessory(Pdu::new(hap_tx)).await?;

	// loop
	task::spawn(mqtt::start(mqtt_host, mqtt_tx, mqtt_rx));

	Ok(async move {
		let state = hap_rx.await?;
		let switches = &state[0]["subdevs"];

		for (i, s) in ptr
			.lock()
			.await
			.get_mut_services()
			.into_iter()
			.filter_map(|service| service.get_mut_characteristic(HapType::PowerState))
			.enumerate()
		{
			if switches[i]["on"].as_u64().unwrap() == 1 {
				s.set_value(true.into()).await?;
			} else {
				s.set_value(false.into()).await?;
			}
		}

		Ok(())
	})
}
