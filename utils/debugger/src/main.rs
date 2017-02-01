extern crate safe_app;
extern crate safe_core;
extern crate clap;

use safe_app::App as SAFEApp;

use safe_core::ipc::decode_msg as decode_ipc_msg;
use std::process;
use clap::{Arg, ArgMatches, App as ClApp, SubCommand};


fn run_decode_ipc(matches: &ArgMatches) -> Result<(), String> {
  let input = matches.value_of("message").unwrap();
  match decode_ipc_msg(input) {
    Ok(decoded) => {
      println!("Message: {:?}", decoded);
      Ok(())
    },
    Err(err) => Err(format!("Couldn't decode: {:?}", err))
  }
}


fn run(matches: ArgMatches) -> Result<(), String> {
  match matches.subcommand() {
    ("decode-ipc", Some(m)) => run_decode_ipc(m,),
    // ("verify", Some(m)) => run_verify(m, &logger),
    _ =>  Err("Not implemented".to_owned()),
  }
  // let client = SAFEApp::unregistered(|_| ());
}

fn main() {
  let matches = ClApp::new("SAFE debugger")
    // meta
    .version("0.1.0")
    .about("Debug your SAFE Network!")
    .author("Benjamin Kampmann")
    // mdata
    // .subcommand(SubCommand::with_name("mGet")
    //   .about("get that mutable data")
    //   .arg(Arg::with_name("address")
    //       .short("a")))

    // ipc
    .subcommand(SubCommand::with_name("decode-ipc")
      .about("decode an ipc message")
      .arg(Arg::with_name("message")
          .short("m")
          .takes_value(true)
          .required(true)))

    .get_matches();

  if let Err(e) = run(matches) {
    println!("Application error: {}", e);
    process::exit(1);
  }
}


