#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;
extern crate clap;

mod mop_structs;
mod mop_act;

use std::env;

use log::{LogRecord, LogLevelFilter};
use env_logger::LogBuilder;
use chrono::prelude::*;
use clap::{Arg, App, SubCommand};

use mop_structs::Metadata as Metadata;

fn init_logging(log_level: &str){
    let format = |record: &LogRecord| {
        let dt = Local::now();
        format!("{} [{}]: {}", dt.format("%Y-%m-%d %H%M%S").to_string(), record.level(), record.args())
    };

    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, LogLevelFilter::Info);
    
    //If building in debug, force debug
    if cfg!(debug_assertions) {
        builder.parse("debug");
    } else {
        builder.parse(log_level);
    }
    builder.init().unwrap();
}

fn main(){
    //Parse command line args and check the sub command
    let args = App::new("MOP - Metadata Organization Program")
                        .version(env!("CARGO_PKG_VERSION"))
                        .author(env!("CARGO_PKG_AUTHORS"))
                        .about("Cleans up messy music files!")
                        .subcommand(SubCommand::with_name("all")
                                    .about("Perform all operations available (excluding 'help' subcommand)"))                       
                        .subcommand(SubCommand::with_name("check")
                                    .about("Verify the given directory and print info about it"))
                        .subcommand(SubCommand::with_name("clean")
                                    .about("Do a full fix of all file metadata"))
                        .subcommand(SubCommand::with_name("cover-art")
                                    .about("Retrieve the cover art for all file, if possible"))
                        .subcommand(SubCommand::with_name("rename")
                                    .about("Rename the file in some specific manner"))
                        .arg(Arg::with_name("directory")
                            .short("i")
                            .long("working-dir")
                            .value_name("PATH")
                            .help("Set the working directory of the program")
                            .required(true)
                            .takes_value(true))
                        .arg(Arg::with_name("log")
                            .short("v")
                            .long("log")
                            .value_name("debug|info|warn|error")
                            .help("Set the logging verbosity of the program (Default: 'warn')")
                            .takes_value(true))
                        .get_matches();

    init_logging(args.value_of("log").unwrap_or("warn"));
    info!("Starting up MOP");

    //TODO: Complete this match!
    match args.subcommand_name() {
        Some("check") => mop_act::quick_check(String::from(args.value_of("directory").unwrap())),
        None        => panic!("No subcommand was used - Not supported yet!"),
        _           => panic!("The subcommand that was used that is not supported yet"),
    }
}