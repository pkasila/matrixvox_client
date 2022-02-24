extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Pack {
    pub anim_rate: usize,
    pub slices: usize,
    pub data: Vec<Vec<[u8; 8]>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceInformation {
    pub product_id: String,
    pub serial_number: String,
    pub vox_size: [i32; 3],
}