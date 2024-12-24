use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
// 下面这些都从 Cargo.toml 中拿
#[command(name="rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to JSON")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    // #[arg(short, long, default_value_t = String::from("output.json"))] // "output.json".into()
    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    // header 以 h 开头，和 help 冲突，可以不使用short，将其规避
    #[arg(long, default_value_t = true)]
    pub header: bool,
}
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        // 使用 into 来进行类型转化，只要filename 实现了 Display trait 就可以
        Ok(filename.into())
    } else {
        Err("file does not exist")
    }
}
