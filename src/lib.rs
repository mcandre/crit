//! crit provides predicates for conveniently managing multiple cross target builds.

extern crate regex;
extern crate toml;

use serde::{Deserialize, Serialize};

use std::collections;
use std::env;
use std::fmt;
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

/// BUILD_MODES enumerates cargo's major build modes.
pub static BUILD_MODES: sync::LazyLock<Vec<&str>> =
    sync::LazyLock::new(|| vec!["debug", "release"]);

/// DEFAULT_BINARY_EXTENSIONS collects common cargo build binary file extensions.
pub static DEFAULT_BINARY_EXTENSIONS: sync::LazyLock<Vec<String>> = sync::LazyLock::new(|| {
    ["", "exe", "js", "wasm"]
        .iter()
        .map(|e| e.to_string())
        .collect()
});

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

/// Target identifies computing platforms.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Target {
    // arch denotes a chipset.
    pub arch: String,

    // vendor denotes a manufacturer.
    pub vendor: String,

    // os denotes an operating system.
    //
    // Warning: `Some("none")` and `None` represent distinct target identifiers.
    pub os: Option<String>,

    // abi denotes a chipset/libc variant.
    pub abi: Option<String>,
}

// RUST_TARGET_PATTERN extracts metadata from Rust target identifiers.
pub static RUST_TARGET_PATTERN: sync::LazyLock<regex::Regex> = sync::LazyLock::new(|| {
    regex::Regex::new(r"^([^\- ]+)-([^\- ]+)(-([^\- ]+)(-([^\- ]+))?)?(\s+\(installed\))?$")
        .unwrap()
});

impl Target {
    /// parse converts Rust target strings to objects.
    pub fn parse(id: &str) -> Result<Target, CritError> {
        let pattern = &RUST_TARGET_PATTERN;

        let m = match pattern.captures(id) {
            Some(e) => e,
            _ => return Err(CritError::IOError(format!("invalid rust target id: {id}"))),
        };

        let arch = m[1].to_string();
        let vendor = m[2].to_string();
        let os = m.get(4).map(|e| e.as_str().to_string());
        let abi = m.get(6).map(|e| e.as_str().to_string());
        Ok(Target {
            arch,
            vendor,
            os,
            abi,
        })
    }
}

#[test]
fn test_target_parsing() -> Result<(), CritError> {
    assert_eq!(
        Target::parse("aarch64-apple-ios-macabi")?,
        Target {
            arch: "aarch64".to_string(),
            vendor: "apple".to_string(),
            os: Some("ios".to_string()),
            abi: Some("macabi".to_string())
        }
    );
    assert_eq!(
        Target::parse("aarch64-apple-darwin")?,
        Target {
            arch: "aarch64".to_string(),
            vendor: "apple".to_string(),
            os: Some("darwin".to_string()),
            abi: None
        }
    );
    assert_eq!(
        Target::parse("aarch64-apple-darwin (installed)")?,
        Target {
            arch: "aarch64".to_string(),
            vendor: "apple".to_string(),
            os: Some("darwin".to_string()),
            abi: None
        }
    );
    assert_eq!(
        Target::parse("wasm32-wasip1")?,
        Target {
            arch: "wasm32".to_string(),
            vendor: "wasip1".to_string(),
            os: None,
            abi: None
        }
    );
    assert!(Target::parse("wasm32").is_err());
    Ok(())
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.arch)?;
        write!(f, "-")?;
        write!(f, "{}", self.vendor)?;

        if let Some(os) = &self.os {
            write!(f, "-")?;
            write!(f, "{}", os)?;

            if let Some(abi) = &self.abi {
                write!(f, "-")?;
                write!(f, "{}", abi)?;
            }
        }

        Ok(())
    }
}

/// get_applications queries Cargo.toml for the list of binary application names.
pub fn get_applications(feature_excludes: &[&str]) -> Result<Vec<String>, CritError> {
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
        .filter(
            |e| match e.get("required-features").and_then(|e2| e2.as_array()) {
                None => true,
                Some(feature_values) => {
                    feature_values
                        .iter()
                        .map(|e2| e2.as_str())
                        .any(|e| match e {
                            Some(feature) => !feature_excludes.contains(&feature),
                            None => false,
                        })
                }
            },
        )
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
#[serde(deny_unknown_fields)]
pub struct Crit {
    /// debug enables additional logging.
    pub debug: Option<bool>,

    /// banner denotes an optional parent directory prefix (e.g. "hello-1.0").
    pub banner: Option<String>,

    /// rustflags maps target triple patterns to custom RUSTFLAGS settings (default: $RUSTFLAGS).
    pub rustflags: Option<collections::BTreeMap<String, String>>,

    /// feature_excludes skips matching features.
    pub feature_excludes: Option<Vec<String>>,

    /// cross_args forwards additional flags to cross.
    pub cross_args: Option<Vec<String>>,

    /// arch collects enabled chipets.
    pub arch: Vec<String>,

    /// vendor collects enabled vendors.
    pub vendor: Vec<String>,

    /// os collects enabled operating systems.
    pub os: Vec<String>,

    /// abi collects enabled chipset/libc variants.
    pub abi: Vec<String>,

    /// target_excludes skips targets.
    pub target_excludes: Option<Vec<String>>,

    /// binary_extensions selects file extensions to collate (default: `DEFAULT_BINARY_EXTENSIONS`).
    pub binary_extensions: Option<Vec<String>>,

    /// targets caches enabled Rust targets.
    #[serde(skip)]
    targets: Option<Vec<Target>>,

    /// enabled_applications caches active applications.
    #[serde(skip)]
    enabled_applications: Option<Vec<String>>,
}

impl Crit {
    /// load generates a Crit.
    pub fn load(pth: &str) -> Result<Self, CritError> {
        let toml_string = fs::read_to_string(pth)
            .map_err(|_| CritError::IOError(format!("unable to read file: {pth}")))?;
        let mut crit: Crit = toml::from_str(&toml_string)
            .map_err(|e| CritError::TOMLParseError(e.message().to_string()))?;
        crit.update_targets()?;
        Ok(crit)
    }

    /// update_targets refreshes the targets cache.
    pub fn update_targets(&mut self) -> Result<(), CritError> {
        let target_excludes = self.target_excludes.clone().unwrap_or_default();

        let mut cmd = process::Command::new("rustup");
        cmd.args(["target", "list"]);

        if let Some(true) = self.debug {
            eprintln!("running command: {:?}", cmd);
        }

        let output = cmd
            .output()
            .map_err(|e| CritError::IOError(e.to_string()))?;

        if !output.status.success() {
            return Err(CritError::IOError(format!(
                "failed to query rustup targets: {}",
                output.status
            )));
        }

        let stdout_utf8 =
            String::from_utf8(output.stdout).map_err(|e| CritError::IOError(e.to_string()))?;
        let mut available_targets: Vec<Target> = Vec::new();

        for line in stdout_utf8.lines() {
            let target = Target::parse(line)?;
            available_targets.push(target);
        }

        let arches = self
            .arch
            .iter()
            .cloned()
            .collect::<collections::HashSet<String>>();
        let vendors = self
            .vendor
            .iter()
            .cloned()
            .collect::<collections::HashSet<String>>();
        let operating_systems = self
            .os
            .iter()
            .map(|e| match e.as_str() {
                "" => None,
                e => Some(e.to_string()),
            })
            .collect::<collections::HashSet<Option<String>>>();
        let abis = self
            .abi
            .iter()
            .map(|e| match e.as_str() {
                "" => None,
                e => Some(e.to_string()),
            })
            .collect::<collections::HashSet<Option<String>>>();

        for arch in &arches {
            if !available_targets.iter().any(|target| target.arch == *arch) {
                return Err(CritError::IOError(format!("invalid arch: {:?}", arch)));
            }
        }

        for vendor in &vendors {
            if !available_targets
                .iter()
                .any(|target| target.vendor == *vendor)
            {
                return Err(CritError::IOError(format!("invalid vendor: {:?}", vendor)));
            }
        }

        for os in &operating_systems {
            if !available_targets.iter().any(|target| target.os == *os) {
                return Err(CritError::IOError(format!("invalid os: {:?}", os)));
            }
        }

        for abi in &abis {
            if !available_targets.iter().any(|target| target.abi == *abi) {
                return Err(CritError::IOError(format!("invalid abi: {:?}", abi)));
            }
        }

        let targets = available_targets
            .into_iter()
            .filter(|target| {
                arches.contains(&target.arch)
                    && vendors.contains(&target.vendor)
                    && operating_systems.contains(&target.os)
                    && abis.contains(&target.abi)
                    && !target_excludes.contains(&target.to_string())
            })
            .collect::<Vec<Target>>();
        self.targets = Some(targets);
        Ok(())
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

        let mut rustflags: Vec<String> = match env::var("RUSTFLAGS") {
            Ok(e) => vec![e],
            _ => Vec::new(),
        };

        for (target_pattern_string, rf) in self.rustflags.clone().unwrap_or_default().into_iter() {
            let target_pattern_regex = regex::Regex::new(&target_pattern_string)
                .map_err(|e| CritError::RegexParseError(e.to_string()))?;

            if target_pattern_regex.is_match(target) {
                rustflags.push(rf);
            }
        }

        cmd.env("RUSTFLAGS", rustflags.join(" "));

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

        let binary_extensions_strings: Vec<String> = self
            .binary_extensions
            .clone()
            .unwrap_or(DEFAULT_BINARY_EXTENSIONS.clone());
        let binary_extensions_strs = binary_extensions_strings
            .iter()
            .map(|e| e.as_str())
            .collect::<Vec<&str>>();

        for application in enabled_applications {
            let dest_dir_pathbuf: path::PathBuf = bin_dir_path.join(target);
            let dest_dir_str: &str = &dest_dir_pathbuf.display().to_string();

            fs::create_dir_all(dest_dir_str).map_err(|err| CritError::IOError(err.to_string()))?;

            for extension in &binary_extensions_strs {
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
        let targets = self.targets.clone().unwrap_or_default();

        if targets.is_empty() {
            eprintln!("warning: empty targets");
            return Ok(());
        }

        let feature_excludes_strings = self.feature_excludes.clone().unwrap_or_default();
        let feature_excludes_strs = feature_excludes_strings
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>();
        self.enabled_applications = Some(get_applications(&feature_excludes_strs)?);

        let bin_dir_pathbuf = if let Some(banner) = &self.banner
            && !banner.is_empty()
        {
            &ARTIFACT_ROOT_PATH.join("bin").join(banner)
        } else {
            &ARTIFACT_ROOT_PATH.join("bin")
        };

        for target in targets {
            eprintln!("building {target}");
            self.build_target(&target.to_string(), bin_dir_pathbuf)?;
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
