//! HAP & MQTT smart majordomo implementation..

#![deny(clippy::all, missing_docs, unused_crate_dependencies)]

mod pdu;
mod util;
mod waker;

mod prelude {
	pub use std::result::Result as StdResult;

	pub use anyhow::Result;

	// std
	use std::sync::Arc;
	// crates.io
	use hap::{accessory::HapAccessory, futures::lock::Mutex, serde_json::Value, server::IpServer};
	use tokio::sync::{mpsc, oneshot};

	pub type OneshotTx = oneshot::Sender<Value>;
	// pub type OneshotRx = oneshot::Receiver<Value>;
	pub type Tx = mpsc::Sender<MqttMessage>;
	pub type Rx = mpsc::Receiver<MqttMessage>;

	pub trait Register {
		async fn register(self, bridge: &IpServer) -> Result<Arc<Mutex<Box<dyn HapAccessory>>>>;
	}

	#[derive(Debug)]
	pub struct MqttMessage {
		pub topic: String,
		pub payload: String,
	}
}
use prelude::*;

// crates.io
use clap::{
	Parser,
	builder::{
		Styles,
		styling::{AnsiColor, Effects},
	},
};
use hap::server::Server;
use tracing_subscriber::fmt;

#[derive(Debug, Parser)]
#[command(
	version = concat!(
		env!("CARGO_PKG_VERSION"),
		"-",
		env!("VERGEN_GIT_SHA"),
		"-",
		env!("VERGEN_CARGO_TARGET_TRIPLE"),
	),
	about,
	rename_all = "kebab",
	styles = styles(),
)]
struct Cli {
	/// Path to the configuration folder.
	#[arg(long, short, value_name = "PATH")]
	configuration: Option<String>,
	/// MQTT host.
	#[arg(long, short, value_name = "IP", default_value_t = String::from("0.0.0.0"))]
	mqtt_host: String,
}

#[tokio::main]
async fn main() -> Result<()> {
	fmt::init();

	let Cli { configuration, mqtt_host } = Cli::parse();
	let bridge = util::bridge(configuration.as_deref()).await?;

	waker::initialize(&bridge).await?;

	let refresh_pdu = pdu::initialize(mqtt_host, &bridge).await?;
	let service = bridge.run_handle();

	refresh_pdu.await?;
	// loop
	service.await.map_err(|e| anyhow::anyhow!("{e}"))?;

	Ok(())
}

fn styles() -> Styles {
	Styles::styled()
		.header(AnsiColor::Red.on_default() | Effects::BOLD)
		.usage(AnsiColor::Red.on_default() | Effects::BOLD)
		.literal(AnsiColor::Blue.on_default() | Effects::BOLD)
		.placeholder(AnsiColor::Green.on_default())
}
