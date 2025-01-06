use clap::Parser;
use rcli::opts::{Opts, SubCommand};
use rcli::process::process_csv;
use rcli::process_gen_pass;

fn main() {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // NOTE: 通过 接收变量的的类型自动识别 ?
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format).expect("TODO: panic message");
        }
        SubCommand::GenPass(opts) => {
            process_gen_pass(
                opts.length,
                opts.lowercase,
                opts.uppercase,
                opts.number,
                opts.symbol,
            );
        }
    }
}
