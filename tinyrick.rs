//! Build configuration

extern crate tinyrick;
extern crate tinyrick_extras;

use std::path;

/// Generate documentation
fn doc() {
    tinyrick_extras::build();
}

/// Security audit
fn audit() {
    tinyrick::deps(cargo_audit);
}

/// Run cargo audit
fn cargo_audit() {
    tinyrick::exec!("cargo", &["audit"]);
}

/// Run clippy
fn clippy() {
    tinyrick_extras::clippy();
}

/// Run rustfmt
fn rustfmt() {
    tinyrick_extras::rustfmt();
}

/// Run unmake
fn unmake() {
    tinyrick::exec!("unmake", &["."]);
    tinyrick::exec!("unmake", &["-n", "."]);
}

/// Validate documentation and run linters
fn lint() {
    tinyrick::deps(doc);
    tinyrick::deps(clippy);
    tinyrick::deps(rustfmt);
    tinyrick::deps(unmake);
}

/// Lint, and then install artifacts
fn install() {
    tinyrick::deps(lint);
    tinyrick::exec!("cargo", &["install", "--force", "--locked", "--path", "."]);
}

/// Uninstall artifacts
fn uninstall() {
    tinyrick::exec!("cargo", &["uninstall"]);
}

/// Run tests
fn test() {
    tinyrick::deps(install);

    assert!(
        tinyrick::exec_mut!("crit", &["-l"])
            .current_dir("example")
            .status()
            .unwrap()
            .success()
    );
}

/// Build: Doc, lint, test, and compile
fn build() {
    tinyrick::deps(lint);
    tinyrick::deps(test);
    tinyrick_extras::build();
}

/// banner generates artifact labels.
fn banner() -> String {
    format!("{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

/// archive bundles executables.
fn archive() {
    tinyrick_extras::archive(
        path::Path::new(".crit").join("bin").display().to_string(),
        banner(),
    );
}

/// Prepare cross-platform release media.
fn port() {
    tinyrick_extras::crit(vec!["-b".to_string(), banner()]);
    tinyrick::deps(archive);
}

/// Publish to crate repository
fn publish() {
    tinyrick_extras::publish();
}

/// Clean example project
fn clean_example() {
    assert!(
        tinyrick::exec_mut!("crit", &["-c"])
            .current_dir("example")
            .status()
            .unwrap()
            .success()
    );
}

/// Clean ports
fn clean_ports() {
    assert!(
        tinyrick::exec_mut!("crit", &["-c"])
            .status()
            .unwrap()
            .success()
    );
}

/// Clean workspaces
fn clean() {
    tinyrick_extras::clean_cargo();
    tinyrick::deps(clean_example);
    tinyrick::deps(clean_ports);
}

/// CLI entrypoint
fn main() {
    tinyrick::phony!(clean);

    tinyrick::wubba_lubba_dub_dub!(
        build;
        doc,
        install,
        uninstall,
        audit,
        cargo_audit,
        clippy,
        rustfmt,
        unmake,
        lint,
        test,
        archive,
        port,
        publish,
        clean_example,
        clean_ports,
        clean
    );
}
