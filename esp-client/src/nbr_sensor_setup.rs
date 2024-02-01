use rf_core::context::NbrSensors;
use rf_distributed::discovery::nbr_sensors_setup::NbrSensorSetup;

pub struct MockSetup;

impl MockSetup {
    pub fn mock_setup() -> Box<dyn NbrSensorSetup> {
        Box::new(MockSetup {})
    }
}

impl NbrSensorSetup for MockSetup {
    fn nbr_sensor_setup(&self, _nbrs: Vec<i32>) -> NbrSensors {
        Default::default()
    }
}
