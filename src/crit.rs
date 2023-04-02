//! CLI crit tool

extern crate getopts;
extern crate lazy_static;
extern crate pad;
extern crate regex;
extern crate toml;

use pad::PadStr;
use std::collections;
use std::env;
use std::fs;
use std::path;
use std::process;

pub static CRIT_ARTIFACT_ROOT : &str = ".crit";

lazy_static::lazy_static! {
    static ref RUSTUP_TARGET_PATTERN : regex::Regex = regex::Regex::new(r"(\S+)").unwrap();

    /// DEFAULT_TARGET_EXCLUSION_PATTERN collects patterns for problematic target triples,
    /// such as bare metal targets that may lack support for the `std` package,
    /// or targets without community supported cross images.
    static ref DEFAULT_TARGET_EXCLUSION_PATTERN : regex::Regex = regex::Regex::new(
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

    /// DEFAULT_FEATURE_EXCLUSION_PATTERN collects patterns for problematic binary features,
    /// such as internal development programs.
    static ref DEFAULT_FEATURE_EXCLUSION_PATTERN : regex::Regex = regex::Regex::new(
        &[
            "letmeout",
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
    eprintln!("{}", (*opts).usage(brief));
}

/// Show version information
pub fn version() {
    eprintln!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

// Query rustup for the list of available Rust target triples
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
                false => Err("error: unable to query rustup target list".to_string()),
                _ => String::from_utf8(output.stdout)
                    .map_err(|_| "error: unable to decode rustup stdout stream".to_string()),
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
    let target_col_header : String = "TARGET".to_string();
    let target_col_header_len = target_col_header.len();

    let mut target_col_values : Vec<&String> = targets
        .keys()
        .collect();
    target_col_values.push(&target_col_header);

    let max_target_len : usize = target_col_values
        .iter()
        .map(|e| e.len())
        .max()
        .unwrap_or(target_col_header_len);

    println!("{} ENABLED\n", target_col_header.pad_to_width(max_target_len));

    for (target, enabled) in targets {
        println!("{} {}", target.pad_to_width(max_target_len), enabled);
    }
}

// Query Cargo.toml for the list of binary application names
pub fn get_applications(feature_exclusion_pattern : regex::Regex) -> Result<Vec<String>, String> {
    let bin_sections_result : Result<Vec<toml::Value>, String> = fs::read_to_string("Cargo.toml")
        .map_err(|_| "error: unable to read Cargo.toml".to_string())
        .and_then(|e|
            e
                .parse::<toml::Table>()
                .map_err(|err| err.to_string())
        )
        .and_then(|e|
            e
                .get("bin")
                .ok_or("error: no binaries declared in Cargo.toml".to_string())
                .map(|e2| e2.clone())
        )
        .and_then(|e|
            e
                .as_array()
                .ok_or("error: binary section not an array in Cargo.toml".to_string())
                .map(|e2| e2.clone())
        );

    let bin_sections : Vec<toml::Value> = bin_sections_result?;

    let name_options : Vec<Option<&toml::Value>> = bin_sections
        .iter()
        .filter(|e| {
            let feature_values_result : Option<&Vec<toml::Value>> = e
                .get("required-features")
                .and_then(|e2| e2.as_array());

            if feature_values_result.is_none() {
                return true
            }

            let feature_values : &Vec<toml::Value> = feature_values_result.unwrap();

            let feature_options : Vec<Option<&str>> = feature_values
                .iter()
                .map(|e2| e2.as_str())
                .collect();

            feature_options
                .iter()
                .any(|e|
                    match e {
                        Some(feature) => feature_exclusion_pattern.is_match(feature),
                        None => false,
                    }
                )
        })
        .map(|e| e.get("name"))
        .collect();

    if name_options.contains(&None) {
        return Err("error: binary missing name field in Cargo.toml".to_string());
    }

    let name_str_results : Vec<Option<&str>> = name_options
        .iter()
        .map(|e| {
            let e2 = e.unwrap();
            e2.as_str()
        })
        .collect();

    if name_str_results.contains(&None) {
        return Err("error: binary name not a string in Cargo.toml".to_string());
    }

    return Ok(
        name_str_results
            .iter()
            .map(|e| e.unwrap())
            .map(|e| e.to_string())
            .collect()
    );
}

pub struct TargetConfig<'a> {
    pub target : &'a str,
    pub cross_dir_pathbuf : &'a path::PathBuf,
    pub bin_dir_pathbuf : &'a path::PathBuf,
    pub cross_args : &'a Vec<String>,
    pub applications: &'a Vec<String>,
}

impl TargetConfig<'_> {
    fn build(&self) -> Result<(), String> {
        let target_dir_pathbuf : path::PathBuf = self.cross_dir_pathbuf
            .join(self.target);

        let target_dir_str : &str = &target_dir_pathbuf
            .display()
            .to_string();

        let cross_output_result : Result<process::Output, String> = process::Command::new("cross")
                .args(["build", "--target-dir", target_dir_str, "--target", self.target])
                .args(self.cross_args.clone())
                .stdout(process::Stdio::piped())
                .stderr(process::Stdio::piped())
                .output()
                .map_err(|err| err.to_string());

        let cross_output : process::Output = cross_output_result?;

        if !cross_output.status.success() {
            let cross_stderr : String = String::from_utf8(cross_output.stderr)
                .map_err(|err| err.to_string())?;

            return Err(cross_stderr);
        }

        for application in self.applications {
            let dest_dir_pathbuf : path::PathBuf = self.bin_dir_pathbuf
                .join(self.target);

            let dest_dir_str : &str = &dest_dir_pathbuf
                .display()
                .to_string();

            fs::create_dir_all(dest_dir_str)
                .map_err(|err| err.to_string())?;

            for extension in BINARY_FILE_EXTENSIONS.iter() {
                for mode in BUILD_MODES.iter() {
                    let mut source_pathbuf : path::PathBuf = target_dir_pathbuf
                        .join(self.target)
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

                        fs::copy(source_str, dest_str)
                            .map_err(|err| err.to_string())?;
                    }
                }
            }
        }

        Ok(())
    }
}

/// clean_resources removes:
///
/// * CRIT_ARTIFACT_ROOT directory
/// * cross Docker containers
///
fn clean_resources(artifact_root_path : &path::Path) -> Result<(), String> {
    let cross_toml_path : &path::Path = path::Path::new("Cross.toml");

    if cross_toml_path.exists() {
        let cross_config : toml::Table = fs::read_to_string("Cross.toml")
            .map_err(|_| "error: unable to read Cross.toml".to_string())
            .and_then(|e|
                e
                    .parse::<toml::Table>()
                    .map_err(|err| err.to_string())
            )?;

        if cross_config.contains_key("target") {
            let blank_table = toml::Value::Table(toml::Table::new());

            let targets_result : Result<&toml::Table, String> = cross_config
                .get("target")
                .unwrap_or(&blank_table)
                .as_table()
                .ok_or("target section not a table in Cross.toml".to_string());

            let targets : &toml::Table = targets_result?;

            let target_options : Vec<Option<&toml::Table>> = targets
                .iter()
                .map(|(_, target)| target.as_table())
                .collect();

            if target_options.iter().any(|e| e.is_none()) {
                return Err("error: target entry not a table in Cross.toml".to_string());
            }

            let image_options : Vec<Option<String>> = target_options
                .iter()
                .map(|e|
                    e
                        .unwrap_or(&toml::Table::new())
                        .get("image")
                        .unwrap_or(&toml::Value::String("".to_string()))
                        .as_str()
                        .map(|e2| e2.to_string())
                )
                .collect();

            if image_options.iter().any(|e| e.is_none()) {
                return Err("error: target image not a string in Cross.toml".to_string());
            }

            let mut images : Vec<String> = image_options
                .iter()
                .map(|e| {
                    let blank_string = "".to_string();

                    e
                        .clone()
                        .unwrap_or(blank_string)
                })
                .collect();

            // cross default image prefix
            images.push("ghcr.io/cross-rs".to_string());

            let docker_ps_output = process::Command::new("docker")
                .args(["ps", "-a"])
                .output()
                .map_err(|_| "error: unable to run docker process list".to_string())?;

            if !docker_ps_output.status.success() {
                let docker_ps_stderr = String::from_utf8(docker_ps_output.stderr)
                    .map_err(|_| "error: unable to decode docker process list stderr stream")?;

                return Err(docker_ps_stderr);
            }

            let docker_ps_stdout : String = String::from_utf8(docker_ps_output.stdout)
                .map_err(|_| "error: unable to decode docker process list stdout stream")?;

            for line in docker_ps_stdout.lines() {
                let pattern = format!("([[:xdigit:]]{{12}})\\s+({})", images.join("|"));

                let re = regex::Regex::new(&pattern)
                    .map_err(|_| "image name introduced invalid Rust regular expression syntax".to_string())?;

                if re.is_match(line) {
                    let container_id : &str = re
                        .captures(line)
                        .and_then(|e| e.get(1))
                        .map(|e| e.as_str())
                        .ok_or("error: container id not a string in docker process list output".to_string())?;

                    let docker_rm_output = process::Command::new("docker")
                        .args(["rm", "-f", container_id])
                        .output()
                        .map_err(|_| "error: unable to run docker container removal".to_string())?;

                    if !docker_rm_output.status.success() {
                        let docker_rm_stderr = String::from_utf8(docker_rm_output.stderr)
                            .map_err(|_| "error: unable to decode docker container removal stderr stream".to_string())?;

                        return Err(docker_rm_stderr);
                    }
                }
            }
        }
    }

    if artifact_root_path.exists() {
        return fs::remove_dir_all(CRIT_ARTIFACT_ROOT)
            .map_err(|_| "error: unable to remove crit artifact root directory".to_string());
    }

    Ok(())
}

/// CLI entrypoint
fn main() {
    let brief = format!("Usage: {} [OPTIONS] [-- <CROSS OPTIONS>]", env!("CARGO_PKG_NAME"));

    let artifact_root_path : &path::Path = path::Path::new(CRIT_ARTIFACT_ROOT);
    let cross_dir_pathbuf : path::PathBuf = artifact_root_path.join("cross");

    let mut clean : bool = false;
    let mut list_targets : bool = false;
    let mut banner : String = "".to_string();
    let mut bin_dir_pathbuf : path::PathBuf = artifact_root_path.join("bin");
    let mut target_exclusion_pattern : regex::Regex = DEFAULT_TARGET_EXCLUSION_PATTERN.clone();
    let mut feature_exclusion_pattern : regex::Regex = DEFAULT_FEATURE_EXCLUSION_PATTERN.clone();
    let mut cross_args : Vec<String> = vec!["-r"]
        .iter()
        .map(|e| e.to_string())
        .collect();

    let mut opts : getopts::Options = getopts::Options::new();
    opts.optflag("c", "clean", "remove artifacts directory and docker containers");
    opts.optopt("b", "banner", "nest artifacts with a further subdirectory label", "<dir>");
    opts.optopt("e", "exclude-targets", "exclude targets", "<rust regex>");
    opts.optopt("F", "exclude-features", "exclude cargo features", "<rust regex>");
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
                clean = true;
            } else if optmatches.opt_present("b") {
                banner = match optmatches.opt_str("b") {
                    None => {
                        eprintln!("error: missing value for banner flag");
                        process::exit(1);
                    },
                    Some(v) => v,
                };
            } else if optmatches.opt_present("e") {
                let ep = match optmatches.opt_str("e") {
                    None => {
                        eprintln!("error: missing value for target exclusion flag");
                        process::exit(1);
                    },
                    Some(v) => v,
                };

                target_exclusion_pattern = match regex::Regex::new(&ep) {
                    Err(err) => {
                        eprintln!("{}", err);
                        process::exit(1);
                    },
                    Ok(v) => v,
                };
            } else if optmatches.opt_present("F") {
                let fp = match optmatches.opt_str("F") {
                    None => {
                        eprintln!("error: missing value for feature exclusion flag");
                        process::exit(1);
                    },
                    Some(v) => v,
                };

                feature_exclusion_pattern = match regex::Regex::new(&fp) {
                    Err(err) => {
                        eprintln!("{}", err);
                        process::exit(1);
                    },
                    Ok(v) => v,
                };
            }

            if arguments.contains(&"--".to_string()) {
                cross_args = optmatches.free;
            }
        }
    }

    if !banner.is_empty() {
        bin_dir_pathbuf = bin_dir_pathbuf.join(banner);
    }

    if clean {
        if let Err(err) = clean_resources(artifact_root_path) {
            eprintln!("{}", err);
            process::exit(1);
        }

        process::exit(0);
    }

    let targets : collections::BTreeMap<String, bool> = match get_targets(target_exclusion_pattern) {
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        },
        Ok(v) => v,
    };

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

    let applications : Vec<String> = match get_applications(feature_exclusion_pattern) {
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        },
        Ok(v) => v,
    };

    for target in enabled_targets {
        let target_config : TargetConfig = TargetConfig{
            target,
            cross_dir_pathbuf: &cross_dir_pathbuf,
            bin_dir_pathbuf: &bin_dir_pathbuf,
            cross_args: &cross_args,
            applications: &applications,
        };

        eprintln!("building {}...", target);

        if let Err(err) = target_config.build() {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}
