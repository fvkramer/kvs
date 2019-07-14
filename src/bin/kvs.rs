extern crate clap;
use clap::{Arg, App, SubCommand}; 
use std::process::exit;

#[allow(unused_imports)]
fn main() {
    let matches = App::new("kvs rust storage")
                        .version(env!("CARGO_PKG_VERSION"))
                        .author(env!("CARGO_PKG_AUTHORS"))
                        .about("Parses CLI args for test db")
                        .subcommand(
                            SubCommand::with_name("get")
                                .arg(Arg::with_name("key"))
                        )
                        .subcommand(
                            SubCommand::with_name("set")
                                .arg(Arg::with_name("key"))
                                .arg(Arg::with_name("value"))
                        )
                        .subcommand(
                            SubCommand::with_name("rm")
                                .arg(Arg::with_name("key"))
                        )
                        .get_matches();

    match matches.subcommand() {
        ("get", Some(get_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("set", Some(set_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(rm_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _   => unreachable!(),
    }
}
