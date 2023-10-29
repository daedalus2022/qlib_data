use clap::Parser;
use qlib_data::command::Opts;

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
