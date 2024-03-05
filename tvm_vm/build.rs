use std::process::Command;

fn get_value(cmd: &str, args: &[&str]) -> String {
    if let Ok(result) = Command::new(cmd).args(args).output() {
        if let Ok(result) = String::from_utf8(result.stdout) {
            return result;
        }
    }
    "Unknown".to_string()
}

fn main() {
    let git_branch = get_value("git", &["rev-parse", "--abbrev-ref", "HEAD"]);
    let git_commit = get_value("git", &["rev-parse", "HEAD"]);
    let commit_date = get_value("git", &["log", "-1", "--date=iso", "--pretty=format:%cd"]);
    let build_time = get_value("date", &["+%Y-%m-%d %T %z"]);
    let rust_version = get_value("rustc", &["--version"]);

    println!("cargo:rustc-env=BUILD_GIT_BRANCH={}", git_branch);
    println!("cargo:rustc-env=BUILD_GIT_COMMIT={}", git_commit);
    println!("cargo:rustc-env=BUILD_GIT_DATE={}", commit_date);
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
    println!("cargo:rustc-env=BUILD_RUST_VERSION={}", rust_version);
}
