// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::opts::{Opts, SubCommand};
use rcli::process::process_csv;

fn main() {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let _ = process_csv(&opts.input, &opts.output);
        }
    }
}
