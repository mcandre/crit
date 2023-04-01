//! CLI crit tool

extern crate getopts;
extern crate lazy_static;
extern crate pad;
extern crate regex;
extern crate toml;

use pad::PadStr;
use std::collections;
use std::env;
use std::io;
use std::fs;
use std::path;
use std::process;
use std::string;

pub static CRIT_ARTIFACT_ROOT : &str = ".crit";

lazy_static::lazy_static! {
    static ref RUSTUP_TARGET_PATTERN : regex::Regex = regex::Regex::new(r"(\S+)").unwrap();

    /// DEFAULT_TARGET_EXCLUSION_PATTERNS collects patterns for problematic target triples,
    /// such as bare metal targets that may lack support for the `std` package,
    /// or targets without community supported cross images.
    static ref DEFAULT_TARGET_EXCLUSION_PATTERNS : regex::Regex = regex::Regex::new(
        &[
            "android",
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
            "uefi",
            "unknown-none",
            "wasm",
        ].join("|")
    ).unwrap();

    static ref BUILD_MODES : Vec<String> = vec![
        "debug",
        "release",
    ]
        .iter()
        .map(|e| e.to_string())
        .collect();

    static ref BINARY_FILE_EXTENSIONS : Vec<String> = vec![
        "",
        "exe",
        "js",
        "wasm",
    ]
        .iter()
        .map(|e| e.to_string())
        .collect();
}

// Show short CLI spec
fn usage(brief : &str, opts : &getopts::Options) {
    println!("{}", (*opts).usage(brief));
}

/// Show version information
pub fn version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

pub fn get_targets(target_exclusion_pattern : regex::Regex) -> Result<collections::BTreeMap<String, bool>, String> {
    return process::Command::new("rustup")
        .args(["target", "list"])
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .output()
        .map_err(|_| "unable to run rustup".to_string())
        .and_then(|output|
            match output.status.success() {
                // work around rustup writing error messages to stdout
                false => Err("unable to query rustup target list".to_string()),
                _ => String::from_utf8(output.stdout)
                    .map_err(|_| "unable to decode rustup stdout stream".to_string()),
            }
        )
        .map(|text|
            text
                .lines()
                .filter(|line| RUSTUP_TARGET_PATTERN.is_match(line))
                .map(|line|
                    RUSTUP_TARGET_PATTERN
                        .captures(line)
                        .and_then(|e| e.get(1))
                        .map(|e| e.as_str())
                        .unwrap()
                )
                .map(|target| (target.to_string(), !target_exclusion_pattern.is_match(target)))
                .collect()
        );
}

/// Render target table
pub fn list(targets : collections::BTreeMap<String, bool>) {
    let max_target_len : usize = targets
        .keys()
        .map(|e| e.len())
        .max()
        .expect("error: all targets blank");

    println!("{} {}\n", "TARGET".pad_to_width(max_target_len), "ENABLED");

    for (target, enabled) in targets {
        println!("{} {}", target.pad_to_width(max_target_len), enabled);
    }
}

struct TargetConfig<'a> {
    cross_dir_pathbuf : &'a path::PathBuf,
    bin_dir_pathbuf : &'a path::PathBuf,
    target : &'a str,
    cross_args : &'a Vec<String>,
    applications: &'a Vec<&'a str>,
}

impl TargetConfig<'_> {
    fn build(&self) -> Result<(), String> {
        let target_dir_pathbuf : path::PathBuf = self.cross_dir_pathbuf
            .join(&self.target);
        let target_dir_str : &str = &target_dir_pathbuf
            .display()
            .to_string();

        let cross_output_result : Result<process::Output, io::Error> = process::Command::new("cross")
                .args(&["build", "--target-dir", target_dir_str, "--target", &self.target])
                .args(self.cross_args.clone())
                .stdout(process::Stdio::piped())
                .stderr(process::Stdio::piped())
                .output();

        if let Err(_) = cross_output_result {
            return Err("unable to run cross".to_string());
        }

        let cross_output : process::Output = cross_output_result.unwrap();

        if !cross_output.status.success() {
            let cross_stderr_result : Result<String, string::FromUtf8Error> = String::from_utf8(cross_output.stderr);

            if let Err(_) = cross_stderr_result {
                return Err("unable to decode cross stderr stream".to_string());
            }

            return Err(cross_stderr_result.unwrap());
        }

        for application in self.applications {
            let dest_dir_pathbuf : path::PathBuf = self.bin_dir_pathbuf
                .join(&self.target);

            let dest_dir_str : &str = &dest_dir_pathbuf
                .display()
                .to_string();

            if let Err(_) = fs::create_dir_all(dest_dir_str) {
                return Err("unable to create bin directory".to_string());
            }

            for extension in BINARY_FILE_EXTENSIONS.iter() {
                for mode in BUILD_MODES.iter() {
                    let mut source_pathbuf : path::PathBuf = target_dir_pathbuf
                        .join(&self.target)
                        .join(mode)
                        .join(application);
                    source_pathbuf.set_extension(extension);

                    if source_pathbuf.exists() {
                        let source_str : &str = &source_pathbuf
                            .display()
                            .to_string();

                        let mut dest_pathbuf : path::PathBuf = dest_dir_pathbuf
                            .join(application);
                        dest_pathbuf.set_extension(extension);

                        let dest_str : &str = &dest_pathbuf
                            .display()
                            .to_string();

                        if let Err(_) = fs::copy(source_str, dest_str) {
                            return Err("unable to copy binary".to_string());
                        }
                    }
                }
            }
        }

        return Ok(())
    }
}

/// CLI entrypoint
fn main() {
    let brief = format!("Usage: {} [OPTIONS] [-- <CROSS OPTIONS>]", env!("CARGO_PKG_NAME"));

    let artifact_root_path : &path::Path = path::Path::new(CRIT_ARTIFACT_ROOT);
    let cross_dir_pathbuf : path::PathBuf = artifact_root_path.join("cross");

    let mut list_targets : bool = false;
    let mut banner : String = "".to_string();
    let mut bin_dir_pathbuf : path::PathBuf = artifact_root_path.join("bin");
    let mut target_exclusion_pattern : regex::Regex = DEFAULT_TARGET_EXCLUSION_PATTERNS.clone();
    let mut cross_args : Vec<String> = vec!["-r"]
        .iter()
        .map(|e| e.to_string())
        .collect();

    let mut opts : getopts::Options = getopts::Options::new();
    opts.optflag("c", "clean", "delete crit artifacts directory tree");
    opts.optopt("b", "banner", "nest artifacts with a further subdirectory label", "<dir>");
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
            if optmatches.opt_present("h") {
                usage(&brief, &opts);
                process::exit(0);
            } else if optmatches.opt_present("v") {
                version();
                process::exit(0);
            } else if optmatches.opt_present("l") {
                list_targets = true;
            } else if optmatches.opt_present("c") {
                if artifact_root_path.exists() {
                    _ = fs::remove_dir_all(CRIT_ARTIFACT_ROOT)
                        .expect("error: unable to delete crit artifact root directory");
                }

                process::exit(0);
            } else if optmatches.opt_present("b") {
                banner = optmatches.opt_str("b")
                    .expect("error: missing value for banner flag");
            } else if optmatches.opt_present("e") {
                let ep = optmatches.opt_str("e")
                    .expect("error: missing value for exclusion flag");

                target_exclusion_pattern = regex::Regex::new(&ep)
                    .expect("error: unable to compile Rust regular expression");
            }

            if arguments.contains(&"--".to_string()) {
                cross_args = optmatches.free;
            }
        }
    }

    if banner != "" {
        bin_dir_pathbuf = bin_dir_pathbuf.join(banner);
    }

    let cargo_str : String = fs::read_to_string("Cargo.toml")
        .expect("error: unable to read Cargo.toml");

    let cargo_table : toml::Table = cargo_str.parse::<toml::Table>()
        .expect("error: unable to parse Cargo.toml");

    let bin_tables : &Vec<toml::Value> = cargo_table["bin"]
        .as_array()
        .expect("error: unable to retrieve bin sections from Cargo.toml");

    let applications : Vec<&str> = bin_tables
        .iter()
        .map(|e|
            e["name"]
                .as_str()
                .expect("error: Cargo.toml binary missing name field")
        )
        .collect();

    if applications.is_empty() {
        eprintln!("error: no binaries declared in Cargo.toml")
    }

    let targets_result : Result<collections::BTreeMap<String, bool>, String> = get_targets(target_exclusion_pattern);

    if let Err(err) = targets_result {
        eprintln!("{}", err);
        process::exit(1);
    }

    let targets : collections::BTreeMap<String, bool> = targets_result.unwrap();

    if list_targets {
        list(targets);
        process::exit(0);
    }

    let enabled_targets : Vec<&str> = targets
        .iter()
        .filter(|(_, &enabled)| enabled)
        .map(|(target, _)| target as &str)
        .collect();

    if enabled_targets.is_empty() {
        eprintln!("error: no targets enabled");
        process::exit(1);
    }

    for target in enabled_targets {
        let target_config : TargetConfig = TargetConfig{
            cross_dir_pathbuf: &cross_dir_pathbuf,
            bin_dir_pathbuf: &bin_dir_pathbuf,
            target: target,
            cross_args: &cross_args,
            applications: &applications,
        };

        println!("building {}...", target);

        if let Err(err) = target_config.build() {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}
