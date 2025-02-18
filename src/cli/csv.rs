// NOTE: 可以通过super对mod中的方法进行引用
use super::verify_input_file;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

// NOTE：Clone 与 Copy 的区别是什么？
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum OutputFormat {
    Json,
    Yaml,
    // Toml,
}
// NOTE: Parser 作用是什么？
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    // #[arg(short, long, default_value_t = String::from("output.json"))] // "output.json".into()
    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    pub output: Option<String>,

    // NOTE: value_parser 作用详解
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    // header 以 h 开头，和 help 冲突，可以不使用short，将其规避
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // NOTE: 如何做的？
    format.parse::<OutputFormat>()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            // OutputFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            // "toml" => Ok(OutputFormat::Toml),
            // NOTE: anyhow! 宏的作用是什么？
            _ => Err(anyhow::anyhow!("Unsupported format 111")),
        }
    }
}

impl fmt::Display for OutputFormat {
    // NOTE: f 是什么类型？
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE: 注意 into 方法的调用方式
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
