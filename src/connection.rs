use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseConnections {
    pub status: i32,
    pub data: Vec<Connection>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionGraphResponse {
    pub status: i32,
    pub data: ConnectionGraphData,
    pub ticket: Ticket,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Connection {
    pub id: String,
    #[serde(rename = "patientId")]
    pub patient_id: String,
    pub country: String,
    pub status: i32,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "targetLow")]
    pub target_low: i32,
    #[serde(rename = "targetHigh")]
    pub target_high: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor {
    #[serde(rename = "deviceId")]
    pub device_id: String,
    pub sn: String,
    pub a: i64,
    pub w: i32,
    pub pt: i32,
    pub s: bool,
    pub lj: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub did: String,
    pub dtid: i32,
    pub v: String,
    pub ll: i32,
    pub hl: i32,
    #[serde(rename = "fixedLowAlarmValues")]
    pub fixed_low_alarm_values: FixedLowAlarmValues,
    pub alarms: bool,
    #[serde(rename = "fixedLowThreshold")]
    pub fixed_low_threshold: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FixedLowAlarmValues {
    pub mgdl: i32,
    pub mmoll: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ticket {
    pub token: String,
    pub expires: u64,
    pub duration: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionGraphData {
    pub connection: Connection,
    #[serde(rename = "activeSensors")]
    pub active_sensors: Vec<ActiveSensor>,
    #[serde(rename = "graphData")]
    pub graph_data: Vec<GraphData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphData {
    #[serde(rename = "FactoryTimestamp")]
    pub factory_timestamp: String,
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
    #[serde(rename = "type")]
    pub _type: i32,
    #[serde(rename = "ValueInMgPerDl")]
    pub value_in_mg_per_dl: i32,
    #[serde(rename = "MeasurementColor")]
    pub measurement_color: i32,
    #[serde(rename = "GlucoseUnits")]
    pub glucose_units: i32,
    #[serde(rename = "Value")]
    pub value: i32,
    #[serde(rename = "isHigh")]
    pub is_high: bool,
    #[serde(rename = "isLow")]
    pub is_low: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveSensor {
    pub sensor: Sensor,
    pub device: Device,
}
