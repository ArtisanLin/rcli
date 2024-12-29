// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::opts::{Opts, SubCommand};
use rcli::process::process_csv;
use anyhow::Context;

fn main() {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // NOTE: 通过 接收变量的的类型自动识别 ?
                "output.json".into()
            };
            process_csv(&opts.input, output, opts.format).with_context(|| "处理参数错误");
        }
    }
}
