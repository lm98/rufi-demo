use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::mqtt::client::MqttClientConfiguration;
use esp_client::{
    discovery::MockDiscovery, 
    nbr_sensor_setup::MockSetup,
};
use esp_rs_wifi::wifi;
use rf_core::context::Context;
use rf_core::sensor_id::{sensor, SensorId};
use rf_distributed::platform::PlatformFactory;
use rf_distributed_esp::mailbox::EspMailbox;
use rf_distributed_esp::network::EspMqttNetwork;
use rufi_gradient::gradient;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("localhost")]
    mqtt_host: &'static str,
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    // The constant `CONFIG` is auto-generated by `toml_config`.
    let app_config = CONFIG;
    // Connect to the Wi-Fi network
    let _wifi = wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    )?;

    //create context
    let self_id = 3;
    let discovery = MockDiscovery::mock_discovery(self_id);
    let nbrs = discovery.discover_neighbors();

    let setup = MockSetup::mock_setup();

    let local_sensor: HashMap<SensorId, Rc<Box<dyn Any>>> = vec![(
        sensor("source"),
        Rc::new(Box::new(false) as Box<dyn Any>),
    )]
    .into_iter()
    .collect();

    let context = Context::new(
        self_id,
        local_sensor.clone(),
        Default::default(),
        Default::default(),
    );

    //setup mailbox
    let mailbox = Box::new(EspMailbox::new());

    //setup mqtt
    let broker = format!("mqtt://{}", app_config.mqtt_host);
    let mqtt_config = MqttClientConfiguration::default();
    let network = Box::new(EspMqttNetwork::new(&broker, &mqtt_config, nbrs));

    //setup platform
    PlatformFactory::sync_platform(mailbox, network, context, discovery, setup).run_forever(gradient).unwrap();
    Ok(())
}
