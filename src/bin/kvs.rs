extern crate clap;
use kvs::{KvStore, KvsError, Result};
use clap::{Arg, App, SubCommand};
use std::env::current_dir;
use std::process::exit;

#[allow(unused_imports)]
fn main() -> Result<()> {
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
        ("get", Some(matches)) => {
            unimplemented!();

        }
        ("set", Some(matches)) => {
            let key = matches.value_of("key").expect("key argument missing");
            let value = matches.value_of("value").expect("value argument missing");

            let mut store = KvStore::open(current_dir()?)?;
            store.set(key.to_string(), value.to_string())?;
        }
        ("rm", Some(matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _   => unreachable!(),
    }
    Ok(())
}
