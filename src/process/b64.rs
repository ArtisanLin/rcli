use std::{fs::File, io::Read};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use crate::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    // NOTE: 处理不同的数据类型, 将不同的类型提升到一个 dynamic trait 下
    // NOTE: dyn 有什么作用？
    // NOTE: 为什么需要Box包裹一层？
    let reader: Box<dyn Read> = if input == "-" {
        // 返回 Stdin
        Box::new(std::io::stdin())
    } else {
        // 返回 File 类型
        Box::new(File::open(input)?)
    };

    let encoded = URL_SAFE.encode(input);
    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str) -> anyhow::Result<()> {
    let decoded = URL_SAFE.decode(input)?;
    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);
    Ok(())
}