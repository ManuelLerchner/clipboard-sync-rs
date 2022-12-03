#![feature(proc_macro_hygiene, decl_macro)]

use clap::{Parser, ValueEnum};
use std::{thread, time::Duration};

mod clipboard;
mod server;
mod transmitter;

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum Mode {
    Sender,
    Receiver,
    Bidirectional,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The mode to run the Server in
    #[clap(value_enum, long, short)]
    sync_mode: Mode,

    /// The url(s) to send the clipboard contents to
    #[clap(long, short, required_if_eq_any([("sync_mode","sender"), ("sync_mode","bidirectional")]))]
    reporting_url: Vec<String>,

    /// The port to listen on
    #[clap(short, long, default_value = "2209")]
    port: u16,

    /// waiting time in ms bevore transmitting
    #[clap(short, long, default_value = "64")]
    wait_time: u64,
}

fn main() {
    let args = Args::parse();

    println!("Starting in '{:?}'-Mode", args.sync_mode);

    let wait_time = Duration::from_millis(args.wait_time);

    //Start correcting functionallity
    match args.sync_mode {
        Mode::Sender => {
            clipboard::register_clipboard_hook(wait_time, args.reporting_url);
        }
        Mode::Receiver => {
            server::start_server(args.port);
        }
        Mode::Bidirectional => {
            clipboard::register_clipboard_hook(wait_time, args.reporting_url);
            server::start_server(args.port);
        }
    }

    // Wait indefinitely
    loop {
        thread::park();
    }
}
