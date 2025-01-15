use clap::Parser;
use rcli::{process_csv, process_decode, process_encode, process_gen_pass, Base64SubCommand, Opts, SubCommand};

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
            )
            .expect("TODO: panic message");
        }

        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format).expect("TODO: panic message")
            }
            
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format).expect("TODO: panic message")
            }
        },
    }
}
