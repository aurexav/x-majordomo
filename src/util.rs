// std
use std::{
	env,
	net::{IpAddr, Ipv4Addr},
};
// crates.io
use hap::{
	Config, MacAddress, Pin,
	accessory::AccessoryCategory,
	server::IpServer,
	storage::{FileStorage, Storage},
};
// self
use crate::prelude::*;

const HOST: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

pub async fn bridge(configuration: Option<&str>) -> Result<IpServer> {
	let mut storage = if let Some(c) = configuration {
		FileStorage::new(c).await?
	} else {
		FileStorage::new(&env::current_dir()?.join("configuration")).await?
	};
	let config = if let Ok(mut c) = storage.load_config().await {
		c.host = HOST;
		storage.save_config(&c).await?;

		c
	} else {
		let c = Config {
			host: HOST,
			pin: Pin::new([0, 0, 0, 0, 0, 0, 0, 1])?,
			name: "x-majordomo".into(),
			device_id: MacAddress::from([0, 0, 0, 0, 0, 1]),
			category: AccessoryCategory::Bridge,
			..Default::default()
		};

		storage.save_config(&c).await?;

		c
	};

	Ok(IpServer::new(config, storage).await?)
}
