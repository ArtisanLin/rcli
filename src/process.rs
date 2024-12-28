use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::{Result, Context};

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    // 使用？来进行错误处理，如果出错就会返回 Err，否则就会返回 Ok
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        // 对 csv 中某一行的处理，可能无法反序列化，所以需要使用 ? 来进行错误处理
        let record: Player = result.with_context(|| "此行无法序列化")?;
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    // 返回 （） unit 元组
    fs::write(output, json)?;

    Ok(())
}
