use std::path::Path;
// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use csv::Reader;
use serde::{Serializer, Deserialize, Serialize};


#[derive(Debug, Parser)]
// 下面这些都从 Cargo.toml 中拿
#[command(name="rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}


#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to JSON")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,

    // #[arg(short, long, default_value_t = String::from("output.json"))] // "output.json".into()
    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    // header 以 h 开头，和 help 冲突，可以不使用short，将其规避
    #[arg(long, default_value_t = true)]
    header: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Player {
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

fn main() {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input).unwrap();
            let records = reader
                .deserialize()
                .map(|record| record.unwrap())
                .collect::<Vec<Player>>();

            println!("{:?}", records)
        }

    }
}

fn verify_input_file(filename: &str) -> Result<String, &'static str>{
    if Path::new(filename).exists() {
        // 使用 into 来进行类型转化，只要filename 实现了 Display trait 就可以
        Ok(filename.into())
    } else {
        Err("file does not exist")
    }
}