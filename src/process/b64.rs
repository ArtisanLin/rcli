use crate::Base64Format;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::{fs::File, io::Read};

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    // NOTE: 处理不同的数据类型, 将不同的类型提升到一个 dynamic trait 下
    // NOTE: dyn 有什么作用？
    // NOTE: 为什么需要Box包裹一层？
    let mut reader: Box<dyn Read> = if input == "-" {
        // 返回 Stdin
        Box::new(std::io::stdin())
    } else {
        // 返回 File 类型
        Box::new(File::open(input)?)
    };

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };

    println!("encoded content: {}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&buf)?,
    };

    // TODO: decode 之后的数据未必就是一个 String
    let decoded = String::from_utf8(decoded)?;
    println!("decoded content: {}", decoded);
    Ok(())
}
