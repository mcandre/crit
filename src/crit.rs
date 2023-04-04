//! CLI crit tool

extern crate crit;
extern crate die;
extern crate getopts;
extern crate lazy_static;
extern crate pad;
extern crate regex;
extern crate toml;

use die::{die, Die};
use std::collections;
use std::env;
use std::path;

/// CLI entrypoint
fn main() {
    let brief: String = format!(
        "Usage: {} [OPTIONS] [-- <CROSS OPTIONS>]",
        env!("CARGO_PKG_NAME")
    );

    let mut opts: getopts::Options = getopts::Options::new();
    opts.optflag(
        "c",
        "clean",
        "remove artifacts directory and docker containers",
    );
    opts.optopt(
        "b",
        "banner",
        "nest artifacts with a further subdirectory label",
        "<dir>",
    );
    opts.optopt("e", "exclude-targets", "exclude targets", "<rust regex>");
    opts.optopt(
        "F",
        "exclude-features",
        "exclude cargo features",
        "<rust regex>",
    );
    opts.optflag("l", "list-targets", "list enabled targets");
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

    let list_targets: bool = optmatches.opt_present("l");
    let clean: bool = optmatches.opt_present("c");

    let banner: String = if !optmatches.opt_present("b") {
        "".to_string()
    } else {
        optmatches
            .opt_str("b")
            .die("error: missing value for banner flag")
    };

    let target_exclusion_pattern: regex::Regex = if !optmatches.opt_present("e") {
        crit::DEFAULT_TARGET_EXCLUSION_PATTERN.clone()
    } else {
        let ep = optmatches
            .opt_str("e")
            .die("error: missing value for target exclusion flag");

        regex::Regex::new(&ep).die("error: unable to parse target exclusion pattern")
    };

    let feature_exclusion_pattern: regex::Regex = if !optmatches.opt_present("F") {
        crit::DEFAULT_FEATURE_EXCLUSION_PATTERN.clone()
    } else {
        let fp = optmatches
            .opt_str("F")
            .die("error: missing value for feature exclusion flag");

        regex::Regex::new(&fp).die("error: unable to parse feature exclusion pattern")
    };

    let cross_args: Vec<String> = if !arguments.contains(&"--".to_string()) {
        vec!["-r".to_string()]
    } else {
        optmatches.free
    };

    let artifact_root_path: &path::Path = path::Path::new(crit::CRIT_ARTIFACT_ROOT);
    let cross_dir_pathbuf: path::PathBuf = artifact_root_path.join("cross");
    let mut bin_dir_pathbuf: path::PathBuf = artifact_root_path.join("bin");

    if !banner.is_empty() {
        bin_dir_pathbuf = bin_dir_pathbuf.join(banner);
    }

    if clean {
        crit::clean(artifact_root_path);
        die!(0);
    }

    let targets: collections::BTreeMap<String, bool> = crit::get_targets(target_exclusion_pattern)
        .die("error: unable to query rustup target list");

    if list_targets {
        let target_table: String =
            crit::format_targets(targets).die("unable to render target table");

        println!("{}", target_table);
        die!(0);
    }

    let applications: Vec<String> = crit::get_applications(feature_exclusion_pattern)
        .die("error: unable to query binary names from Cargo.toml");

    let enabled_targets: Vec<&str> = targets
        .iter()
        .filter(|(_, &enabled)| enabled)
        .map(|(target, _)| target as &str)
        .collect();

    if enabled_targets.is_empty() {
        die!("error: no targets enabled");
    }

    for target in enabled_targets {
        let target_config: crit::TargetConfig = crit::TargetConfig {
            target,
            cross_dir_pathbuf: &cross_dir_pathbuf,
            bin_dir_pathbuf: &bin_dir_pathbuf,
            cross_args: &cross_args,
            applications: &applications,
        };

        eprintln!("building {}...", target);

        if let Err(err) = target_config.build() {
            die!(err);
        }
    }

    eprintln!("artifacts copied to {}", bin_dir_pathbuf.display());
}
