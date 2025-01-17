use crate::Base64Format;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::{fs::File, io::Read};
use anyhow::{ Result};

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };

    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();

    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&buf)?,
    };

    // TODO: decode 之后的数据未必就是一个 String
    let decoded = String::from_utf8(decoded).expect("trans decoded from utf8 failed");
    println!("{}", decoded);
    Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
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
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use crate::Base64Format;

    // NOTE: 需要如此，才能在 tests 模块中使用
    use super::{process_decode, process_encode};
    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        // NOTE: 通过 is_ok 来判断是否成功
        assert!(process_encode(input, format).is_ok())
    }
    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::Standard;
        // NOTE: 通过是 is_ok 来判断是否成功
        assert!(process_decode(input, format).is_ok())
    }

}
