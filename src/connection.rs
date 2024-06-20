use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseConnections {
    pub status: i32,
    pub data: Vec<Connection>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionGraphResponse {
    status: i32,
    data: ConnectionGraphData,
    ticket: Ticket,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Connection {
    id: String,
    #[serde(rename = "patientId")]
    pub patient_id: String,
    country: String,
    status: i32,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(rename = "targetLow")]
    target_low: i32,
    #[serde(rename = "targetHigh")]
    target_high: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor {
    #[serde(rename = "deviceId")]
    device_id: String,
    sn: String,
    a: i64,
    w: i32,
    pt: i32,
    s: bool,
    lj: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    did: String,
    dtid: i32,
    v: String,
    ll: i32,
    hl: i32,
    #[serde(rename = "fixedLowAlarmValues")]
    fixed_low_alarm_values: FixedLowAlarmValues,
    alarms: bool,
    #[serde(rename = "fixedLowThreshold")]
    fixed_low_threshold: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FixedLowAlarmValues {
    mgdl: i32,
    mmoll: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ticket {
    token: String,
    expires: u64,
    duration: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionGraphData {
    connection: Connection,
    #[serde(rename = "activeSensors")]
    active_sensors: Vec<ActiveSensor>,
    #[serde(rename = "graphData")]
    graph_data: Vec<GraphData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphData {
    #[serde(rename = "FactoryTimestamp")]
    factory_timestamp: String,
    #[serde(rename = "Timestamp")]
    timestamp: String,
    #[serde(rename = "type")]
    _type: i32,
    #[serde(rename = "ValueInMgPerDl")]
    value_in_mg_per_dl: i32,
    #[serde(rename = "MeasurementColor")]
    measurement_color: i32,
    #[serde(rename = "GlucoseUnits")]
    glucose_units: i32,
    #[serde(rename = "Value")]
    value: i32,
    #[serde(rename = "isHigh")]
    is_high: bool,
    #[serde(rename = "isLow")]
    is_low: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveSensor {
    sensor: Sensor,
    device: Device,
}
