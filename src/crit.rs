//! CLI crit tool

extern crate crit;
extern crate die;
extern crate getopts;
extern crate regex;

use die::{Die, die};
use std::env;

/// CLI entrypoint
fn main() {
    let brief: String = format!(
        "Usage: {} [OPTIONS] [-- <CROSS OPTIONS>]",
        env!("CARGO_PKG_NAME")
    );

    let mut opts: getopts::Options = getopts::Options::new();
    opts.optopt(
        "b",
        "banner",
        "nest artifacts with a further subdirectory label",
        "<dir>",
    );
    opts.optflag(
        "c",
        "clean",
        "remove artifacts directory and docker containers",
    );
    opts.optflag("d", "debug", "enable additional logging");
    opts.optflag("h", "help", "print usage info");
    opts.optflag("v", "version", "print version info");

    let usage: String = opts.usage(&brief);
    let arguments: Vec<String> = env::args().collect();
    let optmatches: getopts::Matches = opts.parse(&arguments[1..]).die(&usage);

    if optmatches.opt_present("h") {
        die!(0; usage);
    }

    if optmatches.opt_present("v") {
        die!(0; format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
    }

    let debug = optmatches.opt_present("d");

    if optmatches.opt_present("c") {
        if let Err(e) = crit::clean(debug) {
            die!(format!("error: {e}"));
        }

        die!(0);
    }

    let mut c = match crit::Crit::load(crit::CONFIGURATION_FILENAME) {
        Err(e) => die!(1; format!("error: {e}")),
        Ok(e) => e,
    };

    if debug {
        c.debug = Some(true);
    }

    if optmatches.opt_present("b") {
        match optmatches.opt_str("b") {
            None => {
                eprintln!("error: missing value for -b <dir>");
                die!(usage);
            }
            e => c.banner = e,
        };
    };

    if arguments.contains(&"--".to_string()) {
        c.cross_args = Some(optmatches.free.clone());
    }

    if let Err(e) = c.run() {
        die!(1; format!("error: {e}"));
    }
}
