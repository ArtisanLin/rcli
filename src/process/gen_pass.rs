use crate::opts::GenPassOpts;
use anyhow::Result;
use rand::prelude::SliceRandom;
use zxcvbn::zxcvbn;
use rand::thread_rng;
use rand::Rng;

// NOTE: 使用 const 的时候，需要显示声明类型
const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnoprstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_gen_pass(length: u8, lower: bool, upper: bool, number: bool, symbol: bool) -> Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        // NOTE: b 表示什么？字节？
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("SYMBOL wont be empty"))
    }

    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("SYMBOL wont be empty") )
    }

    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("SYMBOL wont be empty") )
    }

    if symbol {
        // NOTE: extend_from_slice 与 push、concat 的区别
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL wont be empty") )
    }

    for _ in 0..(length - password.len() as u8) {
        // NOTE: 理解一下这里的类型转化, expect 是如何工作的
        // NOTE: choose 需要引入 use rand::prelude::SliceRandom;
        let c = chars.choose(&mut rng).expect("chars won't be empty in this context");
        // NOTE: 理解一下这里的 *c 与  as char
        password.push(*c )
    }
    // NOTE: shuffle 方法的作用
    password.shuffle(&mut rng);
    
    let password = String::from_utf8(password)?;
    println!("{}", password);


    // NOTE: password 的强度
    let estimate = zxcvbn(&password, &[]);
    eprintln!("Password strength: {}", estimate.score());

    // TODO：make sure the password has at least one of each type

    Ok(())
}
