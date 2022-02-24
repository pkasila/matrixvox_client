mod client;
mod data;
mod current;

use std::error::Error;
use std::io::{Read, BufRead};
use std::process::{Command, Stdio};
use crate::client::{VoxClient, VoxClientImpl};
use crate::current::BITMAP;
use crate::data::Pack;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting");

    let mut client = VoxClientImpl::new(std::env::args().nth(1).unwrap().to_string()).await.unwrap();

    let mut d: Vec<Vec<[u8; 8]>> = vec![];

    for i in BITMAP {
        let mut v: Vec<[u8; 8]> = vec![];
        for mut j in i {
            v.append(&mut vec![j]);
        }
        d.append(&mut vec![v]);
    }

    client.send_pack(Pack {
        anim_rate: 240,
        slices: 16,
        data: d
    }).await.unwrap();

    return Ok(());
}
