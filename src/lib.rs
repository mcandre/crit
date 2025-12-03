//! crit provides predicates for conveniently managing multiple cross target builds.

extern crate lazy_static;
extern crate pad;
extern crate regex;
extern crate toml;

use pad::PadStr;
use std::collections;
use std::fmt::Write;
use std::fs;
use std::path;
use std::process;
use std::sync;

/// CRIT_ARTIFACT_ROOT denotes the directory housing crit internal files during porting.
pub static CRIT_ARTIFACT_ROOT: &str = ".crit";

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

/// DEFAULT_TARGET_EXCLUSION_PATTERN matches problematic target triples,
/// such as bare metal targets that may lack support for the `std` package,
/// or targets without community supported cross images.
pub static DEFAULT_TARGET_EXCLUSION_PATTERN: sync::LazyLock<regex::Regex> =
    sync::LazyLock::new(|| regex::Regex::new(&FRINGE_TARGETS.join("|")).unwrap());

/// CRATE_FEATURE_EXCLUSIONS collects development applications
/// generally not intended for release.
pub static CRATE_FEATURE_EXCLUSIONS: sync::LazyLock<Vec<&str>> = sync::LazyLock::new(|| {
    vec![
        // tinyrick
        "letmeout",
    ]
});

/// DEFAULT_FEATURE_EXCLUSION_PATTERN matches problematic binary features,
/// such as internal development programs.
pub static DEFAULT_FEATURE_EXCLUSION_PATTERN: sync::LazyLock<regex::Regex> =
    sync::LazyLock::new(|| regex::Regex::new(&CRATE_FEATURE_EXCLUSIONS.join("|")).unwrap());

/// BUILD_MODES enumerates cargo's major build modes.
pub static BUILD_MODES: sync::LazyLock<Vec<&str>> =
    sync::LazyLock::new(|| vec!["debug", "release"]);

/// BINARY_FILE_EXTENSIONS collects potential cargo build binary file extensions.
pub static BINARY_FILE_EXTENSIONS: sync::LazyLock<Vec<&str>> =
    sync::LazyLock::new(|| vec!["", "exe", "js", "wasm"]);

/// get_targets queries rustup for the list of available Rust target triples.
pub fn get_targets(
    target_exclusion_pattern: regex::Regex,
) -> Result<collections::BTreeMap<String, bool>, String> {
    process::Command::new("rustup")
        .args(["target", "list"])
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .output()
        .map_err(|_| "unable to run rustup".to_string())
        .and_then(|output| match output.status.success() {
            // work around rustup writing error messages to stdout
            false => Err("error: unable to query rustup target list".to_string()),
            _ => String::from_utf8(output.stdout)
                .map_err(|_| "error: unable to decode rustup stdout stream".to_string()),
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

/// format_targets renders a target table.
pub fn format_targets(targets: collections::BTreeMap<String, bool>) -> Result<String, String> {
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
    .map_err(|_| "error: unable to render target table format header".to_string())?;

    for (target, enabled) in targets {
        write!(buf, "\n{} {}", target.pad_to_width(max_target_len), enabled)
            .map_err(|_| "error: unable to render target table format row".to_string())?;
    }

    Ok(buf)
}

/// get_applications queries Cargo.toml for the list of binary application names.
pub fn get_applications(feature_exclusion_pattern: regex::Regex) -> Result<Vec<String>, String> {
    let bin_sections_result: Result<Vec<toml::Value>, String> = fs::read_to_string("Cargo.toml")
        .map_err(|_| "error: unable to read Cargo.toml".to_string())
        .and_then(|e| e.parse::<toml::Table>().map_err(|err| err.to_string()))
        .and_then(|e| {
            e.get("bin")
                .ok_or("error: no binaries declared in Cargo.toml".to_string())
                .cloned()
        })
        .and_then(|e| {
            e.as_array()
                .ok_or("error: binary section not an array in Cargo.toml".to_string())
                .cloned()
        });

    let bin_sections: Vec<toml::Value> = bin_sections_result?;

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
        return Err("error: binary missing name field in Cargo.toml".to_string());
    }

    let name_str_results: Vec<Option<&str>> = name_options
        .iter()
        .map(|e| {
            let e2 = e.unwrap();
            e2.as_str()
        })
        .collect();

    if name_str_results.contains(&None) {
        return Err("error: binary name not a string in Cargo.toml".to_string());
    }

    Ok(name_str_results
        .iter()
        .map(|e| e.unwrap())
        .map(|e| e.to_string())
        .collect())
}

/// TargetConfig models a cross build operation.
pub struct TargetConfig<'a> {
    /// target denotes a Rust target triple.
    pub target: &'a str,

    /// cross_dir_pathbuf denotes the cross notion of target directory.
    pub cross_dir_pathbuf: &'a path::PathBuf,

    /// bin_dir_pathbuf denotes the location of a destination directory
    /// for copying artifacts into a recursive archive friendly
    /// subdirectory tree.
    pub bin_dir_pathbuf: &'a path::PathBuf,

    /// cross_args denotes any passthrough arguments to forward to cross.
    pub cross_args: &'a Vec<String>,

    /// applications denotes the names of cargo binaries
    /// expected to be produced during a cross/cargo build.
    pub applications: &'a Vec<String>,
}

impl TargetConfig<'_> {
    /// build executes a cross build.
    pub fn build(&self) -> Result<(), String> {
        let target_dir_pathbuf: path::PathBuf = self.cross_dir_pathbuf.join(self.target);
        let target_dir_str: &str = &target_dir_pathbuf.display().to_string();

        let cross_output_result: Result<process::Output, String> = process::Command::new("cross")
            .args([
                "build",
                "--target-dir",
                target_dir_str,
                "--target",
                self.target,
            ])
            .args(self.cross_args.clone())
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .output()
            .map_err(|err| err.to_string());

        let cross_output: process::Output = cross_output_result?;

        if !cross_output.status.success() {
            let cross_stderr: String =
                String::from_utf8(cross_output.stderr).map_err(|err| err.to_string())?;

            return Err(cross_stderr);
        }

        for application in self.applications {
            let dest_dir_pathbuf: path::PathBuf = self.bin_dir_pathbuf.join(self.target);
            let dest_dir_str: &str = &dest_dir_pathbuf.display().to_string();

            fs::create_dir_all(dest_dir_str).map_err(|err| err.to_string())?;

            for extension in BINARY_FILE_EXTENSIONS.iter() {
                for mode in BUILD_MODES.iter() {
                    let mut source_pathbuf: path::PathBuf = target_dir_pathbuf
                        .join(self.target)
                        .join(mode)
                        .join(application);
                    source_pathbuf.set_extension(extension);

                    if source_pathbuf.exists() {
                        let source_str: &str = &source_pathbuf.display().to_string();

                        let mut dest_pathbuf: path::PathBuf = dest_dir_pathbuf.join(application);
                        dest_pathbuf.set_extension(extension);
                        let dest_str: &str = &dest_pathbuf.display().to_string();

                        fs::copy(source_str, dest_str).map_err(|err| err.to_string())?;
                    }
                }
            }
        }

        Ok(())
    }
}

/// clean_containers removes leftover cross Docker containers.
pub fn clean_containers() -> Result<(), String> {
    let cross_toml_path: &path::Path = path::Path::new("Cross.toml");

    if !cross_toml_path.exists() {
        return Ok(());
    }

    let cross_config: toml::Table = fs::read_to_string("Cross.toml")
        .map_err(|_| "error: unable to read Cross.toml".to_string())
        .and_then(|e| e.parse::<toml::Table>().map_err(|err| err.to_string()))?;

    if !cross_config.contains_key("target") {
        return Ok(());
    }

    let blank_table: toml::Value = toml::Value::Table(toml::Table::new());

    let targets_result: Result<&toml::Table, String> = cross_config
        .get("target")
        .unwrap_or(&blank_table)
        .as_table()
        .ok_or("target section not a table in Cross.toml".to_string());

    let targets: &toml::Table = targets_result?;

    let target_options: Vec<Option<&toml::Table>> = targets
        .iter()
        .map(|(_, target)| target.as_table())
        .collect();

    if target_options.iter().any(|e| e.is_none()) {
        return Err("error: target entry not a table in Cross.toml".to_string());
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
        return Err("error: target image not a string in Cross.toml".to_string());
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

    let docker_ps_output: process::Output = process::Command::new("docker")
        .args(["ps", "-a"])
        .output()
        .map_err(|_| "error: unable to run docker process list".to_string())?;

    if !docker_ps_output.status.success() {
        let docker_ps_stderr = String::from_utf8(docker_ps_output.stderr)
            .map_err(|_| "error: unable to decode docker process list stderr stream")?;

        return Err(docker_ps_stderr);
    }

    let docker_ps_stdout: String = String::from_utf8(docker_ps_output.stdout)
        .map_err(|_| "error: unable to decode docker process list stdout stream")?;

    for line in docker_ps_stdout.lines() {
        let pattern: String = format!("([[:xdigit:]]{{12}})\\s+({})", images.join("|"));

        let re: regex::Regex = regex::Regex::new(&pattern).map_err(|_| {
            "image name introduced invalid Rust regular expression syntax".to_string()
        })?;

        if !re.is_match(line) {
            continue;
        }

        let container_id: &str = re
            .captures(line)
            .and_then(|e| e.get(1))
            .map(|e| e.as_str())
            .ok_or("error: container id not a string in docker process list output".to_string())?;

        let docker_rm_output: process::Output = process::Command::new("docker")
            .args(["rm", "-f", container_id])
            .output()
            .map_err(|_| "error: unable to run docker container removal".to_string())?;

        if !docker_rm_output.status.success() {
            let docker_rm_stderr: String =
                String::from_utf8(docker_rm_output.stderr).map_err(|_| {
                    "error: unable to decode docker container removal stderr stream".to_string()
                })?;

            return Err(docker_rm_stderr);
        }
    }

    Ok(())
}

/// clean_artifact_root removes CRIT_ARTIFACT_ROOT directory.
pub fn clean_artifact_root(artifact_root_path: &path::Path) -> Result<(), String> {
    if !artifact_root_path.exists() {
        return Ok(());
    }

    fs::remove_dir_all(CRIT_ARTIFACT_ROOT)
        .map_err(|_| "error: unable to remove crit artifact root directory".to_string())
}

/// clean removes:
///
/// * cross Docker containers
/// * CRIT_ARTIFACT_ROOT directory
///
pub fn clean(artifact_root_path: &path::Path) {
    _ = clean_containers();
    _ = clean_artifact_root(artifact_root_path);
}
