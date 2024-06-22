use serde::{Deserialize, Serialize};

use crate::connection::Ticket;

#[derive(Serialize, Deserialize, Debug)]
pub struct GlucoseHistoryRequest {
    pub status: i32,
    pub data: GlucoseHistoryData,
    pub ticket: Ticket,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct GlucoseHistoryData {
    #[serde(rename = "lastUpload")]
    pub last_upload: i64,
    #[serde(rename = "lastUploadCGM")]
    pub last_upload_gcm: i64,
    #[serde(rename = "lastUploadPro")]
    pub last_upload_pro: i64,
    #[serde(rename = "reminderSent")]
    pub reminder_sent: i64,
    pub devices: Vec<i32>,
    pub periods: Vec<Period>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Period {
    #[serde(rename = "dateEnd")]
    pub date_end: i64,
    #[serde(rename = "dateStart")]
    pub date_start: i64,
    #[serde(rename = "noData")]
    pub no_data: bool,
    #[serde(rename = "dataType")]
    pub data_type: String,
    #[serde(rename = "avgGlucose")]
    pub avg_glucose: i64,
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "deviceType")]
    pub device_type: i32,
    //mergeable_devices: ?? unkown type
    #[serde(rename = "hypoEvents")]
    pub hypo_events: i32,
    #[serde(rename = "avgTestsPerDay")]
    pub avg_tests_per_day: i32,
    #[serde(rename = "daysOfData")]
    pub days_of_data: i32,
    pub data: PeriodData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PeriodData {
    #[serde(rename = "maxGlucoseRange")]
    pub max_glucose_range: i32,
    #[serde(rename = "minGlucoseRange")]
    pub min_glucose_range: i32,
    #[serde(rename = "maxGlucoseValue")]
    pub max_glucose_value: i32,
    pub blocks: Vec<Vec<Block>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub time: i64,
    pub percentile5: f32,
    pub percentile25: f32,
    pub percentile50: f32,
    pub percentile75: f32,
    pub percentile95: f32,
}
