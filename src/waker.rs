// std
use std::net::SocketAddr;
// crates.io
use hap::{
	accessory::{AccessoryInformation, switch::SwitchAccessory},
	characteristic::CharacteristicCallbacks,
	server::{IpServer, Server},
};
use wakey::WolPacket;
// self
use crate::prelude::*;

struct Waker {
	pc: SwitchAccessory,
}
impl Waker {
	const ID: u8 = 2;
	const NAME: &'static str = "Waker";

	fn new() -> Self {
		let mut pc = SwitchAccessory::new(
			Self::ID as _,
			AccessoryInformation { name: Self::NAME.into(), ..Default::default() },
		)
		.unwrap();

		pc.switch.power_state.on_update(Some(|_: &_, v: &_| {
			if *v {
				let _ = WolPacket::from_bytes(&[0x58, 0x11, 0x22, 0xC0, 0xDE, 0x35])
					.unwrap()
					.send_magic_to(
						SocketAddr::from(([0, 0, 0, 0], 0)),
						SocketAddr::from(([255, 255, 255, 255], 9)),
					);
			}

			Ok(())
		}));

		Self { pc }
	}
}

pub async fn initialize(bridge: &IpServer) -> Result<()> {
	bridge.add_accessory(Waker::new().pc).await?;

	Ok(())
}
