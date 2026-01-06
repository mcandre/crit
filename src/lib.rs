//! crit provides predicates for conveniently managing multiple cross target builds.

extern crate pad;
extern crate regex;
extern crate toml;

use pad::PadStr;
use serde::{Deserialize, Serialize};

use std::collections;
use std::fmt;
use std::fmt::Write;
use std::fs;
use std::path;
use std::process;
use std::sync;

/// CONFIGURATION_FILENAME denotes the file path to an optional TOML configuration file,
/// relative to the current working directory.
pub static CONFIGURATION_FILENAME: &str = "crit.toml";

/// CRIT_ARTIFACT_ROOT denotes the directory housing crit internal files.
pub static CRIT_ARTIFACT_ROOT: &str = ".crit";

/// ARTIFACT_ROOT_PATH denotes the path housing crit internal files.
pub static ARTIFACT_ROOT_PATH: sync::LazyLock<&path::Path> =
    sync::LazyLock::new(|| path::Path::new(CRIT_ARTIFACT_ROOT));

/// CROSS_DIR_PATHBUF denotes the pathbuf housing cross internal files.
pub static CROSS_DIR_PATHBUF: sync::LazyLock<path::PathBuf> =
    sync::LazyLock::new(|| ARTIFACT_ROOT_PATH.join("cross"));

/// RUSTUP_TARGET_PATTERN matches Rust target triples from rustup target list output.
pub static RUSTUP_TARGET_PATTERN: sync::LazyLock<regex::Regex> =
    sync::LazyLock::new(|| regex::Regex::new(r"(\S+)").unwrap());

/// FRINGE_TARGETS collects Rust platform entries
/// likely to not work out of the box.
pub static FRINGE_TARGETS: sync::LazyLock<Vec<&str>> = sync::LazyLock::new(|| {
    vec![
        "android",
        "cuda",
        "emscripten",
        "fortanix",
        "fuchsia",
        "gnullvm",
        "gnux32",
        "i686-pc-windows-gnu",
        "ios",
        "loongarch",
        "msvc",
        "none-eabi",
        "ohos",
        "pc-solaris",
        "powerpc64le-unknown-linux-musl",
        "redox",
        "riscv64gc-unknown-linux-musl",
        "sparcv9-sun-solaris",
        "uefi",
        "unknown-none",
        "wasm",
    ]
});

/// EXCLUSION_TARGETS_PATTERN_REPLACE_TEMPLATE combines `exclusion_targets` and a pipe (|) delimited target string to form a pattern matching skippable targets.
pub static EXCLUSION_TARGETS_PATTERN_REPLACE_TEMPLATE: &str = r"(exclusion_targets)";

/// generate_target_exclusion_pattern builds a target matching pattern from a collection of targets.
pub fn generate_target_exclusion_pattern(targets: &[&str]) -> String {
    EXCLUSION_TARGETS_PATTERN_REPLACE_TEMPLATE.replace("exclusion_targets", &targets.join("|"))
}

#[test]
fn test_default_target_exclusion_pattern() {
    let pattern = regex::Regex::new(&generate_target_exclusion_pattern(&FRINGE_TARGETS)).unwrap();
    assert!(pattern.is_match("aarch64-linux-android"));
    assert!(pattern.is_match("aarch64-apple-ios"));
    assert!(pattern.is_match("i686-pc-windows-gnu"));
    assert!(!pattern.is_match("aarch64-unknown-linux-gnu"));
}

/// DEFAULT_TARGET_EXCLUSION_PATTERN matches problematic target triples,
/// such as bare metal targets that may lack support for the `std` package,
/// or targets without community supported cross images.
pub static DEFAULT_TARGET_EXCLUSION_PATTERN: sync::LazyLock<String> =
    sync::LazyLock::new(|| generate_target_exclusion_pattern(&FRINGE_TARGETS));

/// CRATE_FEATURE_EXCLUSIONS collects development applications
/// generally not intended for release.
pub static CRATE_FEATURE_EXCLUSIONS: sync::LazyLock<Vec<&str>> = sync::LazyLock::new(|| {
    vec![
        // tinyrick
        "letmeout",
    ]
});

/// EXCLUSION_FEATURES_PATTERN_REPLACE_TEMPLATE combines `exclusion_features` and a pipe (|) delimited feature string to form a pattern matching skippable features.
pub static EXCLUSION_FEATURES_PATTERN_REPLACE_TEMPLATE: &str = r"^(exclusion_features)$";

/// generate_feature_exclusion_pattern builds a feature matching pattern from a collection of features.
pub fn generate_feature_exclusion_pattern(features: &[&str]) -> String {
    EXCLUSION_FEATURES_PATTERN_REPLACE_TEMPLATE.replace("exclusion_features", &features.join("|"))
}

#[test]
fn test_default_feature_exclusion_pattern() {
    let pattern = regex::Regex::new(&generate_feature_exclusion_pattern(
        &CRATE_FEATURE_EXCLUSIONS,
    ))
    .unwrap();
    assert!(pattern.is_match("letmeout"));
    assert!(!pattern.is_match("derive"));
}

/// DEFAULT_FEATURE_EXCLUSION_PATTERN matches problematic binary features,
/// such as internal development programs.
pub static DEFAULT_FEATURE_EXCLUSION_PATTERN: sync::LazyLock<String> =
    sync::LazyLock::new(|| generate_feature_exclusion_pattern(&CRATE_FEATURE_EXCLUSIONS));

/// BUILD_MODES enumerates cargo's major build modes.
pub static BUILD_MODES: sync::LazyLock<Vec<&str>> =
    sync::LazyLock::new(|| vec!["debug", "release"]);

/// BINARY_FILE_EXTENSIONS collects potential cargo build binary file extensions.
pub static BINARY_FILE_EXTENSIONS: sync::LazyLock<Vec<&str>> =
    sync::LazyLock::new(|| vec!["", "exe", "js", "wasm"]);

/// CritError models bad computer states.
#[derive(Debug)]
pub enum CritError {
    IOError(String),
    UnsupportedPathError(String),
    PathRenderError(String),
    UnknownMimetypeError(String),
    RegexParseError(String),
    TOMLParseError(String),
}

impl fmt::Display for CritError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CritError::IOError(e) => write!(f, "{e}"),
            CritError::UnknownMimetypeError(e) => write!(f, "{e}"),
            CritError::UnsupportedPathError(e) => write!(f, "{e}"),
            CritError::PathRenderError(e) => write!(f, "{e}"),
            CritError::RegexParseError(e) => write!(f, "{e}"),
            CritError::TOMLParseError(e) => write!(f, "{e}"),
        }
    }
}

impl die::PrintExit for CritError {
    fn print_exit(&self) -> ! {
        eprintln!("{}", self);
        process::exit(die::DEFAULT_EXIT_CODE);
    }
}

/// format_targets renders a target table.
pub fn format_targets(targets: collections::BTreeMap<String, bool>) -> Result<String, CritError> {
    let target_col_header: String = "TARGET".to_string();
    let target_col_header_len: usize = target_col_header.len();

    let mut target_col_values: Vec<&String> = targets.keys().collect();
    target_col_values.push(&target_col_header);

    let max_target_len: usize = target_col_values
        .iter()
        .map(|e| e.len())
        .max()
        .unwrap_or(target_col_header_len);

    let mut buf: String = String::new();
    write!(
        buf,
        "{} ENABLED",
        target_col_header.pad_to_width(max_target_len)
    )
    .map_err(|e| CritError::IOError(format!("unable to render target table format header: {e}")))?;

    for (target, enabled) in targets {
        write!(buf, "\n{} {}", target.pad_to_width(max_target_len), enabled).map_err(|e| {
            CritError::IOError(format!("unable to render target table format row: {e}"))
        })?;
    }

    Ok(buf)
}

/// get_applications queries Cargo.toml for the list of binary application names.
pub fn get_applications(feature_exclusion_pattern: regex::Regex) -> Result<Vec<String>, CritError> {
    let bin_sections: Vec<toml::Value> = fs::read_to_string("Cargo.toml")
        .map_err(|e| CritError::IOError(format!("unable to read Cargo.toml: {e}")))
        .and_then(|e| {
            e.parse::<toml::Table>()
                .map_err(|err| CritError::TOMLParseError(err.to_string()))
        })
        .and_then(|e| {
            e.get("bin")
                .ok_or(CritError::TOMLParseError(
                    "no binaries declared in Cargo.toml".to_string(),
                ))
                .cloned()
        })
        .and_then(|e| {
            e.as_array()
                .ok_or(CritError::TOMLParseError(
                    "binary section not an array in Cargo.toml".to_string(),
                ))
                .cloned()
        })?;

    let name_options: Vec<Option<&toml::Value>> = bin_sections
        .iter()
        .filter(|e| {
            let feature_values_result: Option<&Vec<toml::Value>> =
                e.get("required-features").and_then(|e2| e2.as_array());

            if feature_values_result.is_none() {
                return true;
            }

            let feature_values: &Vec<toml::Value> = feature_values_result.unwrap();

            let feature_options: Vec<Option<&str>> =
                feature_values.iter().map(|e2| e2.as_str()).collect();

            feature_options.iter().any(|e| match e {
                Some(feature) => feature_exclusion_pattern.is_match(feature),
                None => false,
            })
        })
        .map(|e| e.get("name"))
        .collect();

    if name_options.contains(&None) {
        return Err(CritError::TOMLParseError(
            "binary missing name field in Cargo.toml".to_string(),
        ));
    }

    let name_str_results: Vec<Option<&str>> = name_options
        .iter()
        .map(|e| {
            let e2 = e.unwrap();
            e2.as_str()
        })
        .collect();

    if name_str_results.contains(&None) {
        return Err(CritError::TOMLParseError(
            "binary name not a string in Cargo.toml".to_string(),
        ));
    }

    Ok(name_str_results
        .iter()
        .map(|e| e.unwrap())
        .map(|e| e.to_string())
        .collect())
}

/// Crit models a multiplatform build operation.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Crit {
    /// debug enables additional logging.
    pub debug: Option<bool>,

    /// exclusion_targets skips matching targets (default: FRINGE_TARGETS).
    pub exclusion_targets: Option<Vec<String>>,

    /// exclusion_features skips matching features (default: CRATE_FEATURE_EXCLUSIONS).
    pub exclusion_features: Option<Vec<String>>,

    /// banner denotes an optional parent directory prefix (e.g. "hello-1.0").
    pub banner: Option<String>,

    /// cross_args forwards additional flags to cross.
    pub cross_args: Option<Vec<String>>,

    /// enabled_applications caches active applications.
    enabled_applications: Option<Vec<String>>,
}

impl Crit {
    /// load generates a Crit.
    pub fn load(pth: &str) -> Result<Self, CritError> {
        let toml_string = fs::read_to_string(pth)
            .map_err(|_| CritError::IOError(format!("unable to read file: {pth}")))?;
        let crit: Crit = toml::from_str(&toml_string)
            .map_err(|e| CritError::TOMLParseError(e.message().to_string()))?;
        Ok(crit)
    }

    /// get_targets queries rustup for the list of available Rust target triples.
    pub fn get_targets(&self) -> Result<collections::BTreeMap<String, bool>, CritError> {
        let target_exclusion_pattern = regex::Regex::new(&generate_target_exclusion_pattern(
            &self
                .exclusion_targets
                .clone()
                .unwrap_or(
                    FRINGE_TARGETS
                        .clone()
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>(),
                )
                .iter()
                .map(|e| e.as_ref())
                .collect::<Vec<&str>>(),
        ))
        .map_err(|e| CritError::RegexParseError(e.to_string()))?;

        let mut cmd = process::Command::new("rustup");
        cmd.args(["target", "list"]);
        cmd.stdout(process::Stdio::piped());
        cmd.stderr(process::Stdio::piped());

        if let Some(true) = self.debug {
            eprintln!("debug: running command: {:?}", cmd);
        }

        cmd.output()
            .map_err(|e| CritError::IOError(format!("unable to run rustup: {e}")))
            .and_then(|output| match output.status.success() {
                // work around rustup writing error messages to stdout
                false => Err(CritError::IOError(
                    "unable to query rustup target list".to_string(),
                )),
                _ => String::from_utf8(output.stdout).map_err(|e| {
                    CritError::IOError(format!("unable to decode rustup stdout stream: {e}"))
                }),
            })
            .map(|text| {
                text.lines()
                    .filter(|line| RUSTUP_TARGET_PATTERN.is_match(line))
                    .map(|line| {
                        RUSTUP_TARGET_PATTERN
                            .captures(line)
                            .and_then(|e| e.get(1))
                            .map(|e| e.as_str())
                            .unwrap()
                    })
                    .map(|target| {
                        (
                            target.to_string(),
                            !target_exclusion_pattern.is_match(target),
                        )
                    })
                    .collect()
            })
    }

    /// build_target executes a cross build.
    pub fn build_target(&self, target: &str, bin_dir_path: &path::Path) -> Result<(), CritError> {
        let target_dir_pathbuf = &CROSS_DIR_PATHBUF.join(target);
        let target_dir_str: &str = &target_dir_pathbuf.display().to_string();
        let base_args = [
            "build",
            "--target-dir",
            target_dir_str,
            "--target",
            target,

            // Release mode
            "-r",
        ]
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>();
        let extra_args = self.cross_args.clone().unwrap_or_default();
        let args = [base_args, extra_args].concat();

        let mut cmd = process::Command::new("cross");
        cmd.args(args);
        cmd.stdout(process::Stdio::piped());
        cmd.stderr(process::Stdio::piped());

        if let Some(true) = self.debug {
            eprintln!("debug: running command: {:?}", cmd);
        }

        let cross_output: process::Output = cmd
            .output()
            .map_err(|err| CritError::IOError(format!("unable to run cross: {err}")))?;

        if !cross_output.status.success() {
            let cross_stderr: String = String::from_utf8(cross_output.stderr)
                .map_err(|err| CritError::IOError(format!("unable to decode stderr: {err}")))?;

            return Err(CritError::IOError(cross_stderr));
        }

        let enabled_applications: Vec<String> =
            self.enabled_applications.clone().unwrap_or_default();

        for application in enabled_applications {
            let dest_dir_pathbuf: path::PathBuf = bin_dir_path.join(target);
            let dest_dir_str: &str = &dest_dir_pathbuf.display().to_string();

            fs::create_dir_all(dest_dir_str).map_err(|err| CritError::IOError(err.to_string()))?;

            for extension in BINARY_FILE_EXTENSIONS.clone() {
                for mode in BUILD_MODES.clone() {
                    let mut source_pathbuf: path::PathBuf = target_dir_pathbuf
                        .join(target)
                        .join(mode)
                        .join(&application);
                    source_pathbuf.set_extension(extension);

                    if source_pathbuf.exists() {
                        let source_str: &str = &source_pathbuf.display().to_string();

                        let mut dest_pathbuf: path::PathBuf = dest_dir_pathbuf.join(&application);
                        dest_pathbuf.set_extension(extension);
                        let dest_str: &str = &dest_pathbuf.display().to_string();

                        fs::copy(source_str, dest_str)
                            .map_err(|err| CritError::IOError(err.to_string()))?;
                    }
                }
            }
        }

        Ok(())
    }

    /// run builds targets.
    pub fn run(&mut self) -> Result<(), CritError> {
        let targets: collections::BTreeMap<String, bool> = self.get_targets()?;

        let enabled_targets: Vec<&str> = targets
            .iter()
            .filter(|&(_, &enabled)| enabled)
            .map(|(target, _)| target.as_ref())
            .collect();

        if enabled_targets.is_empty() {
            return Err(CritError::IOError("no targets enabled".to_string()));
        }

        let feature_exclusion_pattern = regex::Regex::new(&generate_feature_exclusion_pattern(
            &self
                .exclusion_features
                .clone()
                .unwrap_or(
                    CRATE_FEATURE_EXCLUSIONS
                        .clone()
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>(),
                )
                .iter()
                .map(|e| e.as_ref())
                .collect::<Vec<&str>>(),
        ))
        .map_err(|e| CritError::RegexParseError(e.to_string()))?;

        self.enabled_applications = Some(get_applications(feature_exclusion_pattern)?);

        let bin_dir_pathbuf = if let Some(banner) = &self.banner
            && !banner.is_empty()
        {
            &ARTIFACT_ROOT_PATH.join("bin").join(banner)
        } else {
            &ARTIFACT_ROOT_PATH.join("bin")
        };

        for target in enabled_targets {
            eprintln!("building {}", target);
            self.build_target(target, bin_dir_pathbuf)?;
        }

        eprintln!("artifacts copied to {:?}", bin_dir_pathbuf);

        Ok(())
    }
}

/// clean_containers removes leftover cross Docker containers.
pub fn clean_containers(debug: bool) -> Result<(), CritError> {
    let cross_toml_path: &path::Path = path::Path::new("Cross.toml");

    if !cross_toml_path.exists() {
        return Ok(());
    }

    let cross_config: toml::Table = fs::read_to_string("Cross.toml")
        .map_err(|err| CritError::IOError(format!("unable to read Cross.toml: {err}")))
        .and_then(|e| {
            e.parse::<toml::Table>()
                .map_err(|err| CritError::IOError(err.to_string()))
        })?;

    if !cross_config.contains_key("target") {
        return Ok(());
    }

    let blank_table: toml::Value = toml::Value::Table(toml::Table::new());

    let targets: &toml::Table = cross_config
        .get("target")
        .unwrap_or(&blank_table)
        .as_table()
        .ok_or(CritError::TOMLParseError(
            "target section not a table in Cross.toml".to_string(),
        ))?;

    let target_options: Vec<Option<&toml::Table>> = targets
        .iter()
        .map(|(_, target)| target.as_table())
        .collect();

    if target_options.iter().any(|e| e.is_none()) {
        return Err(CritError::TOMLParseError(
            "target entry not a table in Cross.toml".to_string(),
        ));
    }

    let image_options: Vec<Option<String>> = target_options
        .iter()
        .map(|e| {
            e.unwrap_or(&toml::Table::new())
                .get("image")
                .unwrap_or(&toml::Value::String(String::new()))
                .as_str()
                .map(|e2| e2.to_string())
        })
        .collect();

    if image_options.iter().any(|e| e.is_none()) {
        return Err(CritError::TOMLParseError(
            "target image not a string in Cross.toml".to_string(),
        ));
    }

    let mut images: Vec<String> = image_options
        .iter()
        .map(|e| {
            let blank_string = String::new();
            e.clone().unwrap_or(blank_string)
        })
        .collect();

    // cross default image prefix
    images.push("ghcr.io/cross-rs".to_string());

    let mut cmd_docker_ps = process::Command::new("docker");
    cmd_docker_ps.args(["ps", "-a"]);

    if debug {
        eprintln!("debug: running command: {:?}", cmd_docker_ps);
    }

    let docker_ps_output: process::Output = cmd_docker_ps
        .output()
        .map_err(|err| CritError::IOError(format!("unable to run docker process list: {err}")))?;

    if !docker_ps_output.status.success() {
        let docker_ps_stderr = String::from_utf8(docker_ps_output.stderr).map_err(|err| {
            CritError::IOError(format!(
                "unable to decode docker process list stderr stream: {err}"
            ))
        })?;

        return Err(CritError::IOError(docker_ps_stderr));
    }

    let docker_ps_stdout: String = String::from_utf8(docker_ps_output.stdout).map_err(|err| {
        CritError::IOError(format!(
            "unable to decode docker process list stdout stream: {err}"
        ))
    })?;

    for line in docker_ps_stdout.lines() {
        let pattern: String = format!("([[:xdigit:]]{{12}})\\s+({})", images.join("|"));

        let re: regex::Regex = regex::Regex::new(&pattern).map_err(|err| {
            CritError::RegexParseError(format!(
                "image name introduced invalid Rust regular expression syntax: {err}"
            ))
        })?;

        if !re.is_match(line) {
            continue;
        }

        let container_id: &str = re
            .captures(line)
            .and_then(|e| e.get(1))
            .map(|e| e.as_str())
            .ok_or(CritError::IOError(
                "container id not a string in docker process list output".to_string(),
            ))?;

        let mut cmd_docker_rm = process::Command::new("docker");
        cmd_docker_rm.args(["rm", "-f", container_id]);

        if debug {
            eprintln!("debug: running command: {:?}", cmd_docker_rm);
        }

        let docker_rm_output: process::Output = cmd_docker_rm.output().map_err(|err| {
            CritError::IOError(format!("unable to run docker container removal: {err}"))
        })?;

        if !docker_rm_output.status.success() {
            let docker_rm_stderr: String =
                String::from_utf8(docker_rm_output.stderr).map_err(|err| {
                    CritError::IOError(format!(
                        "unable to decode docker container removal stderr stream: {err}"
                    ))
                })?;

            return Err(CritError::IOError(docker_rm_stderr));
        }
    }

    Ok(())
}

/// clean_artifact_root removes CRIT_ARTIFACT_ROOT directory.
pub fn clean_artifact_root() -> Result<(), CritError> {
    if !&ARTIFACT_ROOT_PATH.exists() {
        return Ok(());
    }

    fs::remove_dir_all(CRIT_ARTIFACT_ROOT).map_err(|err| {
        CritError::IOError(format!(
            "unable to remove crit artifact root directory: {err}"
        ))
    })
}

/// clean removes:
///
/// * cross Docker containers
/// * CRIT_ARTIFACT_ROOT directory
///
pub fn clean(debug: bool) -> Result<(), CritError> {
    clean_containers(debug)?;
    clean_artifact_root()
}
