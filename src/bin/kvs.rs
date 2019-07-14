extern crate clap;
use clap::{Arg, App, SubCommand}; 

fn main() {
    let matches = App::new("kvs rust storage")
                        .version(env!("CARGO_PKG_VERSION"))
                        .author(env!("CARGO_PKG_AUTHORS"))
                        .about("Parses CLI args for test db")
                        .arg(Arg::with_name("get"))
                        .arg(Arg::with_name("set"))
                        .arg(Arg::with_name("remove"))
                        .get_matches();
}
