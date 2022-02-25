mod client;
mod data;
mod current;

use std::error::Error;
use std::io::{Read, BufRead};
use std::process::{Command, Stdio};
use crate::client::{VoxClient, VoxClientImpl};
use crate::data::Pack;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {

    let s_sample = std::env::args().nth(1).unwrap();
    let sample = s_sample.as_str();

    let bitmap = match sample {
        current::circlesoid::NAME => current::circlesoid::BITMAP,
        current::sinusoid::NAME => current::sinusoid::BITMAP,
        current::walking_cube::NAME => current::walking_cube::BITMAP,
        current::text::NAME => current::text::BITMAP,
        &_ => panic!("unknown")
    };

    println!("Starting");

    let mut client = VoxClientImpl::new(std::env::args().nth(2).unwrap().to_string()).await.unwrap();

    let mut d: Vec<Vec<[u8; 8]>> = vec![];

    for i in bitmap {
        let mut v: Vec<[u8; 8]> = vec![];
        for mut j in i {
            v.append(&mut vec![j]);
        }
        d.append(&mut vec![v]);
    }

    client.send_pack(Pack {
        anim_rate: 240,
        slices: 16,
        data: d,
    }).await.unwrap();

    return Ok(());
}
