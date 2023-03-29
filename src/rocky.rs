//! CLI rocky tool

extern crate atomic_option;
extern crate ctrlc;
extern crate getopts;
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;
use std::path;
use std::process;
use std::sync;

pub static ROCKY_ARTIFACT_ROOT : &str = ".rocky";

lazy_static::lazy_static! {
    static ref RUSTUP_TARGET_PATTERN : regex::Regex = regex::Regex::new(r"(\S+)").unwrap();

    static ref DEFAULT_TARGET_EXCLUSION_PATTERNS : regex::Regex = regex::Regex::new(
        &[
            "aarch64-pc-windows-msvc",
            "emscripten",
            "ios",
            "fuchsia",
            "none-eabi",
            "i586-pc-windows-msvc",
            "i686-pc-windows-msvc",
            "nvidia-cuda",
            "unknown-none",
            "wasm32",
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

    let terminating1 : sync::Arc<sync::atomic::AtomicBool> = sync::Arc::new(sync::atomic::AtomicBool::new(false));
    let terminating2 : sync::Arc<sync::atomic::AtomicBool> = terminating1.clone();

    let child1 : sync::Arc<atomic_option::AtomicOption<process::Child>> = sync::Arc::new(atomic_option::AtomicOption::empty());
    let child2 : sync::Arc<atomic_option::AtomicOption<process::Child>> = child1.clone();

    ctrlc::set_handler(move || {
        terminating2.store(true, sync::atomic::Ordering::Relaxed);

        if let Some(mut c) = child2.take(sync::atomic::Ordering::Relaxed) {
            c
                .kill()
                .unwrap();
            c
                .wait()
                .unwrap();
        }

        process::exit(1);
    }).expect("error registering signal handler");

    let artifact_root : &path::Path = path::Path::new(ROCKY_ARTIFACT_ROOT);
    let mut target_exclusion_pattern : regex::Regex = DEFAULT_TARGET_EXCLUSION_PATTERNS.clone();
    let list_targets : bool;
    let arguments : Vec<String> = env::args().collect();
    let rest : Vec<String>;
    let mut opts : getopts::Options = getopts::Options::new();
    opts.optflag("c", "clean", "delete .rocky artifacts directory");
    opts.optopt("e", "exclude-targets", "exclude targets", "<rust regex>");
    opts.optflag("l", "list-targets", "list enabled targets");
    opts.optflag("h", "help", "print usage info");
    opts.optflag("v", "version", "print version info");

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
                    _ = fs::remove_dir_all(ROCKY_ARTIFACT_ROOT).unwrap();
                }

                process::exit(0);
            } else if optmatches.opt_present("e") {
                let ep = optmatches.opt_str("e").unwrap();
                target_exclusion_pattern = regex::Regex::new(&ep).unwrap();
            }

            rest = optmatches.free;
        }
    }

    let mut rustup_command : process::Command = process::Command::new("rustup");
    rustup_command.args(["target", "list"]);

    if terminating1.load(sync::atomic::Ordering::Relaxed) {
        process::exit(1);
    }

    let rustup_child : process::Child = rustup_command
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()
        .unwrap();

    _ = child1.swap(Box::new(rustup_child), sync::atomic::Ordering::Relaxed);
    let rustup_box : Box<process::Child> = child1.take(sync::atomic::Ordering::Relaxed).unwrap();
    let rustup_output : process::Output = rustup_box.wait_with_output().unwrap();

    if !rustup_output.status.success() {
        println!("{}", String::from_utf8(rustup_output.stderr).unwrap());
        process::exit(1);
    }

    let rustup_target_text : String = String::from_utf8(rustup_output.stdout).unwrap();
    let mut targets : Vec<&str> = Vec::new();

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

        if !target_exclusion_pattern.is_match(target) {
            targets.push(target)
        }
    }

    if list_targets {
        for target in targets {
            println!("{}", target);
        }

        process::exit(0);
    }

    if targets.is_empty() {
        println!("no targets enabled");
        process::exit(1);
    }

    for target in targets {
        if terminating1.load(sync::atomic::Ordering::Relaxed) {
            process::exit(1);
        }

        let target_dir : &str = &artifact_root
            .join(target)
            .display()
            .to_string();

        println!("building {}...", target);

        let cross_child : process::Child = process::Command::new("cross")
                .args(&["build", "--target-dir", target_dir, "--target", target])
                .args(rest.clone())
                .stdout(process::Stdio::piped())
                .stderr(process::Stdio::piped())
                .spawn()
                .unwrap();

        _ = child1.swap(Box::new(cross_child), sync::atomic::Ordering::Relaxed);
        let cross_box : Box<process::Child> = child1.take(sync::atomic::Ordering::Relaxed).unwrap();
        let cross_output : process::Output = cross_box.wait_with_output().unwrap();

        if !cross_output.status.success() {
            println!("{}", String::from_utf8(cross_output.stderr).unwrap());
            process::exit(1);
        }
    }
}
