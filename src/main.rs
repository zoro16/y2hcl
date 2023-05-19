extern crate serde_yaml;

use clap::Parser;
use std::fs::read_to_string;

use y2hcl::run;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Y2hclArgs {
    /// Filename or full path to YAML formated Helm Chart values
    #[arg(short, long)]
    filename: String,

    /// Output format is how we want our Helm Chart values to look like.
    /// Supported format are `hcl_map`, `set_value`, `sensitive_value`, `helm_cli`
    #[arg(short, long)]
    output_format: String,
}

fn main() {
    let args = Y2hclArgs::parse();

    let contents = read_to_string(args.filename).expect("Unable to read file");
    let data: serde_yaml::Value = serde_yaml::from_str(&contents).expect("Failed to parse YAML");

    run(data, args.output_format);
}
