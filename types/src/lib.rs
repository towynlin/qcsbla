use quic_rpc::rpc_service;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorReading(pub f32);

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetTemp(pub f32);

rpc_service! {
    Request = ThermostatRequest;
    Response = ThermostatResponse;
    Service = ThermostatService;
    CreateDispatch = create_thermostat_dispatch;

    Rpc sensor_reading = SensorReading, _ -> TargetTemp;
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
