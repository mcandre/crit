//! CLI crit tool

extern crate getopts;
extern crate lazy_static;
extern crate pad;
extern crate regex;

use pad::PadStr;
use std::cmp;
use std::collections;
use std::env;
use std::fs;
use std::path;
use std::process;

pub static CRIT_ARTIFACT_ROOT : &str = ".crit";

lazy_static::lazy_static! {
    static ref RUSTUP_TARGET_PATTERN : regex::Regex = regex::Regex::new(r"(\S+)").unwrap();

    /// DEFAULT_TARGET_EXCLUSION_PATTERNS collects patterns for problematic target triples,
    /// such as bare metal targets that may lack support for the `std` package,
    /// or targets without community supported cross images.
    static ref DEFAULT_TARGET_EXCLUSION_PATTERNS : regex::Regex = regex::Regex::new(
        &[
            "cuda",
            "emscripten",
            "fortanix",
            "fuchsia",
            "gnux32",
            "ios",
            "msvc",
            "none-eabi",
            "pc-solaris",
            "redox",
            "unknown-none",
            "wasm",
        ].join("|")
    ).unwrap();
}

// Show short CLI spec
fn usage(brief : &str, opts : &getopts::Options) {
    println!("{}", (*opts).usage(brief));
}

/// Show version information
pub fn banner() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

/// CLI entrypoint
fn main() {
    let brief = format!("Usage: {} [OPTIONS] [-- <CROSS OPTIONS>]", env!("CARGO_PKG_NAME"));
    let artifact_root : &path::Path = path::Path::new(CRIT_ARTIFACT_ROOT);
    let mut target_exclusion_pattern : regex::Regex = DEFAULT_TARGET_EXCLUSION_PATTERNS.clone();
    let list_targets : bool;
    let mut rest : Vec<String> = vec!["-r"]
        .iter()
        .map(|e| e.to_string())
        .collect();

    let mut opts : getopts::Options = getopts::Options::new();
    opts.optflag("c", "clean", "delete crit artifacts directory tree");
    opts.optopt("e", "exclude-targets", "exclude targets", "<rust regex>");
    opts.optflag("l", "list-targets", "list enabled targets");
    opts.optflag("h", "help", "print usage info");
    opts.optflag("v", "version", "print version info");
    let arguments : Vec<String> = env::args().collect();

    match opts.parse(&arguments[1..]) {
        Err(_) => {
            usage(&brief, &opts);
            process::exit(1);
        },
        Ok(optmatches) => {
            list_targets = optmatches.opt_present("l");

            if optmatches.opt_present("h") {
                usage(&brief, &opts);
                process::exit(0);
            } else if optmatches.opt_present("v") {
                banner();
                process::exit(0);
            } else if optmatches.opt_present("c") {
                if artifact_root.exists() {
                    _ = fs::remove_dir_all(CRIT_ARTIFACT_ROOT).unwrap();
                }

                process::exit(0);
            } else if optmatches.opt_present("e") {
                let ep = optmatches.opt_str("e").unwrap();
                target_exclusion_pattern = regex::Regex::new(&ep).unwrap();
            }

            if !optmatches.free.is_empty() {
                rest = optmatches.free;
            }
        }
    }

    let rustup_output : process::Output = process::Command::new("rustup")
        .args(["target", "list"])
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .output()
        .unwrap();

    if !rustup_output.status.success() {
        println!("{}", String::from_utf8(rustup_output.stderr).unwrap());
        process::exit(1);
    }

    let rustup_target_text : String = String::from_utf8(rustup_output.stdout).unwrap();
    let mut targets : collections::BTreeMap<&str, bool> = collections::BTreeMap::new();

    for line in rustup_target_text.lines() {
        if !RUSTUP_TARGET_PATTERN.is_match(line) {
            continue
        }

        let target : &str = RUSTUP_TARGET_PATTERN
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        let enabled : bool = !target_exclusion_pattern.is_match(target);
        targets.insert(target, enabled);
    }

    if list_targets {
        let mut max_target_len : usize = 0;

        for target in targets.keys() {
            let target_len : usize = target.len();
            max_target_len = cmp::max(max_target_len, target_len);
        }

        println!("{} {}\n", "TARGET".pad_to_width(max_target_len), "ENABLED");

        for (target, enabled) in targets {
            println!("{} {}", target.pad_to_width(max_target_len), enabled);
        }

        process::exit(0);
    }

    targets.retain(|_, &mut enabled| enabled);

    if targets.is_empty() {
        println!("no targets enabled");
        process::exit(1);
    }

    for target in targets.keys() {
        println!("building {}...", target);

        let target_dir : &str = &artifact_root
            .join(target)
            .display()
            .to_string();

        let cross_output : process::Output = process::Command::new("cross")
                .args(&["build", "--target-dir", target_dir, "--target", target])
                .args(rest.clone())
                .stdout(process::Stdio::piped())
                .stderr(process::Stdio::piped())
                .output()
                .unwrap();

        if !cross_output.status.success() {
            println!("{}", String::from_utf8(cross_output.stderr).unwrap());
            process::exit(1);
        }
    }
}
